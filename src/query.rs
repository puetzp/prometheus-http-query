use crate::client::Client;
use crate::error::BuilderError;
use crate::response::instant::InstantQueryResponse;
use crate::response::range::RangeQueryResponse;
use async_trait::async_trait;
use std::fmt;
use std::str::FromStr;
use time::OffsetDateTime;

#[async_trait]
pub trait Query<T: for<'de> serde::Deserialize<'de>> {
    fn get_query_params(&self) -> Vec<(&str, &str)>;
    fn get_query_endpoint(&self) -> &str;

    /// Execute a query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, RangeQuery, InstantQuery};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = RangeQuery {
    ///     query: "up",
    ///     start: "2021-04-09T11:30:00.000+02:00",
    ///     end: "2021-04-09T12:30:00.000+02:00",
    ///     step: "5m",
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    ///
    /// let query = InstantQuery {
    ///     query: "up".to_string(),
    ///     time: None,
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    async fn execute(&self, client: &Client) -> Result<T, reqwest::Error> {
        let mut url = client.base_url.clone();

        url.push_str(self.get_query_endpoint());

        let params = self.get_query_params();

        let response = client
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        match response.error_for_status() {
            Ok(res) => res.json::<T>().await,
            Err(err) => Err(err),
        }
    }
}

pub struct InstantQuery {
    pub query: String,
    pub time: Option<String>,
    pub timeout: Option<String>,
}

#[async_trait]
impl Query<InstantQueryResponse> for InstantQuery {
    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("query", self.query.as_str())];

        if let Some(t) = &self.time {
            params.push(("time", t.as_str()));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout", t.as_str()));
        }

        params
    }

    fn get_query_endpoint(&self) -> &str {
        "/query"
    }
}

impl InstantQuery {
    pub fn builder(&self) -> InstantQueryBuilder {
        InstantQueryBuilder {
            ..Default::default()
        }
    }
}

pub struct RangeQuery<'a> {
    pub query: &'a str,
    pub start: &'a str,
    pub end: &'a str,
    pub step: &'a str,
    pub timeout: Option<&'a str>,
}

#[async_trait]
impl<'a> Query<RangeQueryResponse> for RangeQuery<'a> {
    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![
            ("query", self.query),
            ("start", self.start),
            ("end", self.end),
            ("step", self.step),
        ];

        if let Some(t) = &self.timeout {
            params.push(("timeout", t));
        }

        params
    }

    fn get_query_endpoint(&self) -> &str {
        "/query_range"
    }
}

pub struct InstantQueryBuilder<'b> {
    metric: Option<&'b str>,
    labels: Option<Vec<Label<'b>>>,
    time: Option<Time>,
    timeout: Option<Vec<Duration>>,
}

impl<'b> Default for InstantQueryBuilder<'b> {
    fn default() -> Self {
        InstantQueryBuilder {
            metric: None,
            labels: None,
            time: None,
            timeout: None,
        }
    }
}

impl<'b> InstantQueryBuilder<'b> {
    pub fn metric(mut self, metric: &'b str) -> Result<Self, BuilderError> {
        match metric {
            "bool" | "on" | "ignoring" | "group_left" | "group_right" => {
                Err(BuilderError::InvalidMetricName)
            }
            _ => {
                self.metric = Some(metric);
                Ok(self)
            }
        }
    }

    pub fn with_label(mut self, label: &'b str, value: &'b str) -> Self {
        if let Some(ref mut labels) = self.labels {
            labels.push(Label::With((label, value)));
        } else {
            self.labels = Some(vec![Label::With((label, value))]);
        }

        self
    }

    pub fn without_label(mut self, label: &'b str, value: &'b str) -> Self {
        if let Some(ref mut labels) = self.labels {
            labels.push(Label::Without((label, value)));
        } else {
            self.labels = Some(vec![Label::Without((label, value))]);
        }

        self
    }

    pub fn match_label(mut self, label: &'b str, value: &'b str) -> Self {
        if let Some(ref mut labels) = self.labels {
            labels.push(Label::Matches((label, value)));
        } else {
            self.labels = Some(vec![Label::Matches((label, value))]);
        }

        self
    }

    pub fn no_match_label(mut self, label: &'b str, value: &'b str) -> Self {
        if let Some(ref mut labels) = self.labels {
            labels.push(Label::Clashes((label, value)));
        } else {
            self.labels = Some(vec![Label::Matches((label, value))]);
        }

        self
    }

    pub fn at(mut self, time: &'b str) -> Result<Self, BuilderError> {
        match f64::from_str(time) {
            Ok(t) => self.time = Some(Time::Unix(t)),
            Err(_) => match OffsetDateTime::parse(time, "%FT%T%z") {
                Ok(t) => self.time = Some(Time::Rfc3339(t)),
                Err(_) => return Err(BuilderError::InvalidTimeSpecifier),
            },
        }
        Ok(self)
    }

    pub fn timeout(mut self, timeout: &'b str) -> Result<Self, BuilderError> {
        let chars = ['s', 'm', 'h', 'd', 'w', 'y'];

        let durations: Result<Vec<Duration>, BuilderError> = timeout
            .split_inclusive(chars.as_ref())
            .map(|d| {
                if d.ends_with("ms") {
                    match d.strip_suffix("ms").unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Milliseconds(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('s') {
                    match d.strip_suffix('s').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Seconds(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('m') {
                    match d.strip_suffix('m').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Minutes(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('h') {
                    match d.strip_suffix('h').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Hours(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('d') {
                    match d.strip_suffix('d').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Days(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('w') {
                    match d.strip_suffix('w').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Weeks(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else if d.ends_with('y') {
                    match d.strip_suffix('y').unwrap().parse::<usize>() {
                        Ok(num) => Ok(Duration::Years(num)),
                        Err(_) => Err(BuilderError::InvalidTimeDuration),
                    }
                } else {
                    return Err(BuilderError::InvalidTimeDuration);
                }
            })
            .collect();

        if let Ok(mut d) = durations {
            d.sort_unstable();
            self.timeout = Some(d);
        }

        Ok(self)
    }

    pub fn build(&self) -> Result<InstantQuery, BuilderError> {
        let time = match &self.time {
            Some(t) => match t {
                Time::Unix(t) => Some(t.to_string()),
                Time::Rfc3339(t) => Some(t.format("%FT%T%z")),
            },
            None => None,
        };

        let timeout = match &self.timeout {
            Some(to) => {
                let formatted = to
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .as_slice()
                    .concat();

                Some(formatted)
            }
            None => None,
        };

        let labels = match &self.labels {
            Some(l) => {
                let joined = l
                    .iter()
                    .map(|x| match x {
                        Label::With(pair) => format!("{}={}", pair.0, pair.1),
                        Label::Without(pair) => format!("{}!={}", pair.0, pair.1),
                        Label::Matches(pair) => format!("{}=~{}", pair.0, pair.1),
                        Label::Clashes(pair) => format!("{}!~{}", pair.0, pair.1),
                    })
                    .collect::<Vec<String>>()
                    .as_slice()
                    .join(",");

                Some(joined)
            }
            None => None,
        };

        let query = match self.metric {
            Some(m) => match labels {
                Some(l) => format!("{}{{{}}}", m, l),
                None => m.to_string(),
            },
            None => match labels {
                Some(l) => format!("{{{}}}", l),
                None => return Err(BuilderError::InvalidQuery),
            },
        };

        let q = InstantQuery {
            query: query,
            time: time,
            timeout: timeout,
        };

        Ok(q)
    }
}

enum Label<'c> {
    With((&'c str, &'c str)),
    Without((&'c str, &'c str)),
    Matches((&'c str, &'c str)),
    Clashes((&'c str, &'c str)),
}

enum Time {
    Unix(f64),
    Rfc3339(OffsetDateTime),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Duration {
    Milliseconds(usize),
    Seconds(usize),
    Minutes(usize),
    Hours(usize),
    Days(usize),
    Weeks(usize),
    Years(usize),
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Milliseconds(d) => write!(f, "{}ms", d),
            Duration::Seconds(d) => write!(f, "{}s", d),
            Duration::Minutes(d) => write!(f, "{}m", d),
            Duration::Hours(d) => write!(f, "{}h", d),
            Duration::Days(d) => write!(f, "{}d", d),
            Duration::Weeks(d) => write!(f, "{}w", d),
            Duration::Years(d) => write!(f, "{}y", d),
        }
    }
}
