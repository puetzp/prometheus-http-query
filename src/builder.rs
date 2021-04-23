use crate::error::BuilderError;
use crate::query::{InstantQuery, RangeQuery};
use chrono::DateTime;
use std::fmt;
use std::str::FromStr;

mod private {
    pub trait SealedQueryBuilder {}

    impl SealedQueryBuilder for super::InstantQueryBuilder<'_> {}
    impl SealedQueryBuilder for super::RangeQueryBuilder<'_> {}
}

pub trait QueryBuilder<'b>: private::SealedQueryBuilder {
    #[doc(hidden)]
    fn get_metric(&self) -> Option<&'b str>;
    #[doc(hidden)]
    fn set_metric(&mut self, metric: &'b str);
    #[doc(hidden)]
    fn get_labels(&self) -> Option<&Vec<Label<'b>>>;
    #[doc(hidden)]
    fn set_label(&mut self, label: Label<'b>);
    #[doc(hidden)]
    fn set_labels(&mut self, labels: Vec<Label<'b>>);
    #[doc(hidden)]
    fn get_timeout(&self) -> Option<&Vec<Duration>>;
    #[doc(hidden)]
    fn set_timeout(&mut self, timeout: Vec<Duration>);

    /// Add a metric name to the time series selector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("up")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    ///
    /// Some strings are reserved PromQL keywords and cannot be used in a query (at least not
    /// as a metric name except using the `__name__` label like `{__name__="on"}`).
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, BuilderError, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder().metric("group_left");
    ///
    /// assert!(query.is_err());
    /// ```
    fn metric(mut self, metric: &'b str) -> Result<Self, BuilderError>
    where
        Self: Sized,
    {
        match metric {
            "bool" | "on" | "ignoring" | "group_left" | "group_right" => {
                Err(BuilderError::InvalidMetricName)
            }
            _ => {
                self.set_metric(metric);
                Ok(self)
            }
        }
    }

    /// Add a label matcher that only selects labels that exactly match the provided string.
    /// Label matchers are chainable and label names can even appear multiple times in one query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "200")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn with_label(mut self, label: &'b str, value: &'b str) -> Self
    where
        Self: Sized,
    {
        if self.get_labels().is_some() {
            self.set_label(Label::With((label, value)));
        } else {
            self.set_labels(vec![Label::With((label, value))]);
        }

        self
    }

    /// Add a label matcher that only selects labels that do not match the provided string.
    /// Label matchers are chainable and label names can even appear multiple times in one query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .without_label("code", "500")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn without_label(mut self, label: &'b str, value: &'b str) -> Self
    where
        Self: Sized,
    {
        if self.get_labels().is_some() {
            self.set_label(Label::Without((label, value)));
        } else {
            self.set_labels(vec![Label::Without((label, value))]);
        }

        self
    }

    /// Add a label matcher that only selects labels that regex-match the provided string.
    /// Label matchers are chainable and label names can even appear multiple times in one query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .match_label("code", "400|500")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn match_label(mut self, label: &'b str, value: &'b str) -> Self
    where
        Self: Sized,
    {
        if self.get_labels().is_some() {
            self.set_label(Label::Matches((label, value)));
        } else {
            self.set_labels(vec![Label::Matches((label, value))]);
        }

        self
    }

    /// Add a label matcher that only selects labels that do not regex-match the provided string.
    /// Label matchers are chainable and label names can even appear multiple times in one query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .no_match_label("code", "400|500")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn no_match_label(mut self, label: &'b str, value: &'b str) -> Self
    where
        Self: Sized,
    {
        if self.get_labels().is_some() {
            self.set_label(Label::Clashes((label, value)));
        } else {
            self.set_labels(vec![Label::Matches((label, value))]);
        }

        self
    }

    /// Provide a custom evaluation timeout other than the Prometheus server's
    /// default. Must adhere to the PromQL [time duration format](https://prometheus.io/docs/prometheus/latest/querying/basics/#time_durations).
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "200")
    ///     .timeout("30s500ms")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn timeout(mut self, timeout: &'b str) -> Result<Self, BuilderError>
    where
        Self: Sized,
    {
        let chars = ['s', 'm', 'h', 'd', 'w', 'y'];

        let durations: Result<Vec<Duration>, BuilderError> = timeout
            .split_inclusive(chars.as_ref())
            .map(|s| s.split_inclusive("ms"))
            .flatten()
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
            self.set_timeout(d);
        }

        Ok(self)
    }

    type Output;

    fn build(&self) -> Result<Self::Output, BuilderError>;
}

#[derive(Debug)]
pub enum Label<'c> {
    With((&'c str, &'c str)),
    Without((&'c str, &'c str)),
    Matches((&'c str, &'c str)),
    Clashes((&'c str, &'c str)),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Duration {
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

#[derive(Debug)]
pub struct InstantQueryBuilder<'b> {
    pub(crate) metric: Option<&'b str>,
    pub(crate) labels: Option<Vec<Label<'b>>>,
    pub(crate) time: Option<String>,
    pub(crate) timeout: Option<Vec<Duration>>,
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

impl<'b> QueryBuilder<'b> for InstantQueryBuilder<'b> {
    fn get_metric(&self) -> Option<&'b str> {
        self.metric
    }

    fn set_metric(&mut self, metric: &'b str) {
        self.metric = Some(metric)
    }

    fn get_labels(&self) -> Option<&Vec<Label<'b>>> {
        self.labels.as_ref()
    }

    fn set_label(&mut self, label: Label<'b>) {
        if let Some(ref mut l) = &mut self.labels {
            l.push(label)
        }
    }

    fn set_labels(&mut self, labels: Vec<Label<'b>>) {
        self.labels = Some(labels)
    }

    fn get_timeout(&self) -> Option<&Vec<Duration>> {
        self.timeout.as_ref()
    }

    fn set_timeout(&mut self, timeout: Vec<Duration>) {
        self.timeout = Some(timeout)
    }

    type Output = InstantQuery;

    /// Build the query using the provided parameters.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "400")
    ///     .with_label("code", "500")
    ///     .at("1618987524")
    ///     .unwrap()
    ///     .timeout("1m30s500ms")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn build(&self) -> Result<InstantQuery, BuilderError> {
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
                        Label::With(pair) => format!("{}=\"{}\"", pair.0, pair.1),
                        Label::Without(pair) => format!("{}!=\"{}\"", pair.0, pair.1),
                        Label::Matches(pair) => format!("{}=~\"{}\"", pair.0, pair.1),
                        Label::Clashes(pair) => format!("{}!~\"{}\"", pair.0, pair.1),
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
                None => return Err(BuilderError::IllegalVectorSelector),
            },
        };

        let q = InstantQuery {
            query: query,
            time: self.time.clone(),
            timeout: timeout,
        };

        Ok(q)
    }
}

impl<'a> InstantQueryBuilder<'a> {
    /// Evaluate a query at a specific point in time. `time` must be either a UNIX timestamp
    /// with optional decimal places or a RFC3339-compatible timestamp which is passed to the
    /// function as a string literal, e.g. `1618922012` or `2021-04-20T14:33:32+02:00`.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "200")
    ///     .at("1618922012")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    ///
    /// let another_query = InstantQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "200")
    ///     .at("2021-04-20T14:33:32+02:00")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let another_response = tokio_test::block_on( async { another_query.execute(&client).await.unwrap() });
    /// assert!(another_response.is_success());
    /// ```
    pub fn at(mut self, time: &'a str) -> Result<Self, BuilderError> {
        match f64::from_str(time) {
            Ok(t) => self.time = Some(t.to_string()),
            Err(_) => match DateTime::parse_from_rfc3339(time) {
                Ok(t) => self.time = Some(t.to_rfc3339()),
                Err(_) => return Err(BuilderError::InvalidTimeSpecifier),
            },
        }
        Ok(self)
    }
}

#[derive(Debug)]
pub struct RangeQueryBuilder<'b> {
    pub(crate) metric: Option<&'b str>,
    pub(crate) labels: Option<Vec<Label<'b>>>,
    pub(crate) start: Option<String>,
    pub(crate) end: Option<String>,
    pub(crate) step: Option<Vec<Duration>>,
    pub(crate) timeout: Option<Vec<Duration>>,
}

impl<'b> Default for RangeQueryBuilder<'b> {
    fn default() -> Self {
        RangeQueryBuilder {
            metric: None,
            labels: None,
            start: None,
            end: None,
            step: None,
            timeout: None,
        }
    }
}

impl<'b> QueryBuilder<'b> for RangeQueryBuilder<'b> {
    fn get_metric(&self) -> Option<&'b str> {
        self.metric
    }

    fn set_metric(&mut self, metric: &'b str) {
        self.metric = Some(metric)
    }

    fn get_labels(&self) -> Option<&Vec<Label<'b>>> {
        self.labels.as_ref()
    }

    fn set_label(&mut self, label: Label<'b>) {
        if let Some(ref mut l) = &mut self.labels {
            l.push(label)
        }
    }

    fn set_labels(&mut self, labels: Vec<Label<'b>>) {
        self.labels = Some(labels)
    }

    fn get_timeout(&self) -> Option<&Vec<Duration>> {
        self.timeout.as_ref()
    }

    fn set_timeout(&mut self, timeout: Vec<Duration>) {
        self.timeout = Some(timeout)
    }

    type Output = RangeQuery;

    /// Build the query using the provided parameters.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, RangeQuery, QueryBuilder};
    ///
    /// let client: Client = Default::default();
    ///
    /// let query = RangeQuery::builder()
    ///     .metric("promhttp_metric_handler_requests_total")
    ///     .unwrap()
    ///     .with_label("code", "400")
    ///     .with_label("code", "500")
    ///     .start("1618987524")
    ///     .unwrap()
    ///     .end("1619166669")
    ///     .unwrap()
    ///     .step("5m")
    ///     .unwrap()
    ///     .timeout("30s")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(response.is_success());
    /// ```
    fn build(&self) -> Result<RangeQuery, BuilderError> {
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

        let step = match &self.step {
            Some(to) => to
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .as_slice()
                .concat(),
            None => return Err(BuilderError::IllegalRangeQuery),
        };

        let labels = match &self.labels {
            Some(l) => {
                let joined = l
                    .iter()
                    .map(|x| match x {
                        Label::With(pair) => format!("{}=\"{}\"", pair.0, pair.1),
                        Label::Without(pair) => format!("{}!=\"{}\"", pair.0, pair.1),
                        Label::Matches(pair) => format!("{}=~\"{}\"", pair.0, pair.1),
                        Label::Clashes(pair) => format!("{}!~\"{}\"", pair.0, pair.1),
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
                None => return Err(BuilderError::IllegalVectorSelector),
            },
        };

        let start = match &self.start {
            Some(s) => s.clone(),
            None => return Err(BuilderError::IllegalRangeQuery),
        };

        let end = match &self.end {
            Some(e) => e.clone(),
            None => return Err(BuilderError::IllegalRangeQuery),
        };

        let q = RangeQuery {
            query,
            start,
            end,
            step,
            timeout,
        };

        Ok(q)
    }
}

impl<'a> RangeQueryBuilder<'a> {
    pub fn start(mut self, start: &'a str) -> Result<Self, BuilderError> {
        match f64::from_str(start) {
            Ok(t) => self.start = Some(t.to_string()),
            Err(_) => match DateTime::parse_from_rfc3339(start) {
                Ok(t) => self.start = Some(t.to_rfc3339()),
                Err(_) => return Err(BuilderError::InvalidTimeSpecifier),
            },
        }
        Ok(self)
    }

    pub fn end(mut self, end: &'a str) -> Result<Self, BuilderError> {
        match f64::from_str(end) {
            Ok(t) => self.end = Some(t.to_string()),
            Err(_) => match DateTime::parse_from_rfc3339(end) {
                Ok(t) => self.end = Some(t.to_rfc3339()),
                Err(_) => return Err(BuilderError::InvalidTimeSpecifier),
            },
        }
        Ok(self)
    }

    pub fn step(mut self, step: &'a str) -> Result<Self, BuilderError> {
        let chars = ['s', 'm', 'h', 'd', 'w', 'y'];

        let durations: Result<Vec<Duration>, BuilderError> = step
            .split_inclusive(chars.as_ref())
            .map(|s| s.split_inclusive("ms"))
            .flatten()
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
            self.step = Some(d);
        }

        Ok(self)
    }
}
