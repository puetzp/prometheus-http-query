use crate::error::{ApiError, Error, MissingFieldError};
use crate::response::*;
use crate::selector::Selector;
use crate::util::{build_final_url, RuleType, TargetState, ToBaseUrl};
use reqwest::Method as HttpMethod;
use std::collections::HashMap;
use url::Url;

/// A builder object used to set some query parameters in the context
/// of an instant query before sending the query on its way.
#[derive(Clone)]
pub struct InstantQueryBuilder {
    client: Client,
    base_url: Url,
    query: String,
    time: Option<i64>,
    timeout: Option<i64>,
    stats: bool,
}

impl InstantQueryBuilder {
    fn build_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![("query", self.query.to_string())];

        if let Some(t) = self.time {
            params.push(("time", t.to_string()));
        }

        if let Some(t) = self.timeout {
            params.push(("timeout", format!("{}ms", t)));
        }

        if self.stats {
            params.push(("stats", String::from("all")));
        }

        params
    }

    /// Set the evaluation timestamp (Unix timestamp in seconds, e.g. 1659182624).
    /// If this is not set the evaluation timestamp will default to the current Prometheus
    /// server time.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    pub fn at(mut self, time: i64) -> Self {
        self.time = Some(time);
        self
    }

    /// Set the evaluation timeout (milliseconds, e.g. 1000).
    /// If this is not set the timeout will default to the value of the "-query.timeout" flag of the Prometheus server.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Instruct Prometheus to compile query statistics.
    pub fn stats(mut self) -> Self {
        self.stats = true;
        self
    }

    /// Execute the instant query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<QueryResult, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/query");
        self.client
            .send(url, Some(self.build_params()), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(convert_query_response)
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<QueryResult, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/query");
        self.client
            .send(url, Some(self.build_params()), HttpMethod::POST)
            .await
            .and_then(check_api_response)
            .and_then(convert_query_response)
    }
}

/// A builder object used to set some query parameters in the context
/// of a range query before sending the query on its way.
#[derive(Clone)]
pub struct RangeQueryBuilder {
    client: Client,
    base_url: Url,
    query: String,
    start: i64,
    end: i64,
    step: f64,
    timeout: Option<i64>,
    stats: bool,
}

impl RangeQueryBuilder {
    fn build_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![
            ("query", self.query.to_string()),
            ("start", self.start.to_string()),
            ("end", self.end.to_string()),
            ("step", self.step.to_string()),
        ];

        if let Some(t) = self.timeout {
            params.push(("timeout", format!("{}ms", t)));
        }

        if self.stats {
            params.push(("stats", String::from("all")));
        }

        params
    }

    /// Set the evaluation timeout (milliseconds, e.g. 1000).
    /// If this is not set the timeout will default to the value of the "-query.timeout" flag of the Prometheus server.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#range-queries)
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Instruct Prometheus to compile query statistics.
    pub fn stats(mut self) -> Self {
        self.stats = true;
        self
    }

    /// Execute the range query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<QueryResult, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/query_range");
        self.client
            .send(url, Some(self.build_params()), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(convert_query_response)
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<QueryResult, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/query_range");
        self.client
            .send(url, Some(self.build_params()), HttpMethod::POST)
            .await
            .and_then(check_api_response)
            .and_then(convert_query_response)
    }
}

/// A client used to execute queries. It uses a [reqwest::Client] internally
/// that manages connections for us.
#[derive(Clone)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: Url,
}

impl Default for Client {
    /// Create a standard Client that sends requests to "http://127.0.0.1:9090/".
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// let client = Client::default();
    /// ```
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            base_url: Url::parse("http://127.0.0.1:9090/").unwrap(),
        }
    }
}

impl std::str::FromStr for Client {
    type Err = crate::error::Error;

    /// Create a Client from a custom base URL. Note that the API-specific
    /// path segments (like `/api/v1/query`) are added automatically.
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    /// use std::str::FromStr;
    ///
    /// let client = Client::from_str("http://proxy.example.com/prometheus");
    /// assert!(client.is_ok());
    /// ```
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let client = Client {
            base_url: url.to_base_url()?,
            client: reqwest::Client::new(),
        };
        Ok(client)
    }
}

impl std::convert::TryFrom<&str> for Client {
    type Error = crate::error::Error;

    /// Create a [Client] from a custom base URL. Note that the API-specific
    /// path segments (like `/api/v1/query`) are added automatically.
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    /// use std::convert::TryFrom;
    ///
    /// let client = Client::try_from("http://proxy.example.com/prometheus");
    /// assert!(client.is_ok());
    /// ```
    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let client = Client {
            base_url: url.to_base_url()?,
            client: reqwest::Client::new(),
        };
        Ok(client)
    }
}

impl std::convert::TryFrom<String> for Client {
    type Error = crate::error::Error;

    /// Create a [Client] from a custom base URL. Note that the API-specific
    /// path segments (like `/api/v1/query`) are added automatically.
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    /// use std::convert::TryFrom;
    ///
    /// let url = String::from("http://proxy.example.com/prometheus");
    /// let client = Client::try_from(url);
    /// assert!(client.is_ok());
    /// ```
    fn try_from(url: String) -> Result<Self, Self::Error> {
        let client = Client {
            base_url: url.to_base_url()?,
            client: reqwest::Client::new(),
        };
        Ok(client)
    }
}

impl Client {
    /// Return a reference to the wrapped [reqwest::Client], i.e. to
    /// use it for other requests unrelated to the Prometheus API.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     // An amittedly bad example, but that is not the point.
    ///     let response = client
    ///         .inner()
    ///         .head("http://127.0.0.1:9090")
    ///         .send()
    ///         .await
    ///         .map_err(Error::Client)?;
    ///
    ///     // Prometheus does not allow HEAD requests.
    ///     assert_eq!(response.status(), reqwest::StatusCode::METHOD_NOT_ALLOWED);
    ///     Ok(())
    /// }
    /// ```
    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }

    /// Return a reference to the base URL that is used in requests to
    /// the Prometheus API.
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    /// use std::str::FromStr;
    ///
    /// let client = Client::default();
    ///
    /// assert_eq!(client.base_url().as_str(), "http://127.0.0.1:9090/");
    ///
    /// let client = Client::from_str("https://proxy.example.com:8443/prometheus").unwrap();
    ///
    /// assert_eq!(client.base_url().as_str(), "https://proxy.example.com:8443/prometheus");
    /// ```
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Create a Client from a custom [reqwest::Client] and URL.
    /// This way you can account for all extra parameters (e.g. x509 authentication)
    /// that may be needed to connect to Prometheus or an intermediate proxy,
    /// by building it into the [reqwest::Client].
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = {
    ///         let c = reqwest::Client::builder()
    ///             .no_proxy()
    ///             .build()
    ///             .map_err(Error::Client)?;
    ///         Client::from(c, "https://prometheus.example.com")
    ///     };
    ///
    ///     assert!(client.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub fn from(client: reqwest::Client, url: &str) -> Result<Self, Error> {
        let base_url = url.to_base_url()?;
        Ok(Client { base_url, client })
    }

    /// Build and send the final HTTP request. Parse the result as JSON.
    async fn send(
        &self,
        url: Url,
        params: Option<Vec<(&str, String)>>,
        method: HttpMethod,
    ) -> Result<ApiResponse, Error> {
        let request = match method {
            HttpMethod::GET => {
                let mut req = self.client.get(url);
                if let Some(p) = params {
                    req = req.query(p.as_slice());
                }
                req
            }
            HttpMethod::POST => {
                let mut req = self.client.post(url);
                if let Some(p) = params {
                    req = req.form(p.as_slice());
                }
                req
            }
            _ => unreachable!(),
        };

        request
            .send()
            .await
            .map_err(Error::Client)?
            .error_for_status()
            .map_err(Error::Client)?
            .json::<ApiResponse>()
            .await
            .map_err(Error::Client)
    }

    /// Create an [InstantQueryBuilder] from a PromQL query allowing you to set some query parameters
    /// (e.g. evaluation timeout) before finally sending the instant query to the server.
    ///
    /// # Arguments
    /// * `query` - PromQL query to exeute
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    ///
    /// ```rust
    /// use prometheus_http_query::{Error, Client};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.query("prometheus_http_request_total").get().await?;
    ///
    ///     assert!(response.as_instant().is_some());
    ///
    ///     // Or make a POST request.
    ///     let response = client.query("prometheus_http_request_total").post().await?;
    ///
    ///     assert!(response.as_instant().is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn query(&self, query: impl std::string::ToString) -> InstantQueryBuilder {
        InstantQueryBuilder {
            client: self.clone(),
            base_url: self.base_url.clone(),
            query: query.to_string(),
            time: None,
            timeout: None,
            stats: false,
        }
    }

    /// Create a [RangeQueryBuilder] from a PromQL query allowing you to set some query parameters
    /// (e.g. evaluation timeout) before finally sending the range query to the server.
    ///
    /// # Arguments
    /// * `query` - PromQL query to exeute
    /// * `start` - Start timestamp as Unix timestamp (seconds)
    /// * `end` - End timestamp as Unix timestamp (seconds)
    /// * `step` - Query resolution step width as float number of seconds
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#range-queries)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let q = "prometheus_http_requests_total";
    ///
    ///     let response = client.query_range(q, 1648373100, 1648373300, 10.0).get().await?;
    ///
    ///     assert!(response.as_range().is_some());
    ///
    ///     // Or make a POST request.
    ///     let response = client.query_range(q, 1648373100, 1648373300, 10.0).post().await?;
    ///
    ///     assert!(response.as_range().is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn query_range(
        &self,
        query: impl std::string::ToString,
        start: i64,
        end: i64,
        step: f64,
    ) -> RangeQueryBuilder {
        RangeQueryBuilder {
            client: self.clone(),
            base_url: self.base_url.clone(),
            query: query.to_string(),
            timeout: None,
            stats: false,
            start,
            end,
            step,
        }
    }

    /// Find time series that match certain label sets ([Selector]s).
    ///
    /// # Arguments
    /// * `selectors` - List of [Selector]s that select the series to return. Must not be empty.
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#finding-series-by-label-matchers)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let s1 = Selector::new()
    ///         .eq("handler", "/api/v1/query");
    ///
    ///     let s2 = Selector::new()
    ///         .eq("job", "node")
    ///         .regex_eq("mode", ".+");
    ///
    ///     let response = client.series(&[s1, s2], None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn series(
        &self,
        selectors: &[Selector<'_>],
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<HashMap<String, String>>, Error> {
        if selectors.is_empty() {
            return Err(Error::EmptySeriesSelector);
        }

        let url = build_final_url(self.base_url.clone(), "api/v1/series");

        let mut params = vec![];

        if let Some(s) = start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = end {
            params.push(("end", e.to_string()));
        }

        let mut matchers: Vec<(&str, String)> = selectors
            .iter()
            .map(|s| ("match[]", s.to_string()))
            .collect();

        params.append(&mut matchers);

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve label names.
    ///
    /// # Arguments
    /// * `selectors` - List of [Selector]s to restrict the set of time series to read the label names from. Optional.
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#getting-label-names)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     // To retrieve a list of all labels:
    ///     let response = client.label_names(None, None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Use a selector to retrieve a list of labels that appear in specific time series:
    ///     let s1 = Selector::new()
    ///         .eq("handler", "/api/v1/query");
    ///
    ///     let s2 = Selector::new()
    ///         .eq("job", "node")
    ///         .regex_eq("mode", ".+");
    ///
    ///     let set = Some(vec![s1, s2]);
    ///
    ///     let response = client.label_names(set, None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn label_names(
        &self,
        selectors: Option<Vec<Selector<'_>>>,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<String>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/labels");

        let mut params = vec![];

        if let Some(s) = &start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = &end {
            params.push(("end", e.to_string()));
        }

        if let Some(items) = selectors {
            let mut matchers: Vec<(&str, String)> =
                items.iter().map(|s| ("match[]", s.to_string())).collect();

            params.append(&mut matchers);
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve all label values for a specific label name.
    ///
    /// # Arguments
    /// * `label` - Name of the label to return all occuring label values for.
    /// * `selectors` - List of [Selector]s to restrict the set of time series to read the label values from. Optional.
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-label-values)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     // To retrieve a list of all label values for a specific label name:
    ///     let response = client.label_values("job", None, None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of label values of labels that appear in specific time series:
    ///     let s1 = Selector::new()
    ///         .regex_eq("instance", ".+");
    ///
    ///     let set = Some(vec![s1]);
    ///
    ///     let response = client.label_values("job", set, None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn label_values(
        &self,
        label: &str,
        selectors: Option<Vec<Selector<'_>>>,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<String>, Error> {
        let path = format!("api/v1/label/{}/values", label);
        let url = build_final_url(self.base_url.clone(), &path);

        let mut params = vec![];

        if let Some(s) = &start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = &end {
            params.push(("end", e.to_string()));
        }

        if let Some(items) = selectors {
            let mut matchers: Vec<(&str, String)> =
                items.iter().map(|s| ("match[]", s.to_string())).collect();

            params.append(&mut matchers);
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Query the current state of target discovery.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#targets)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error, TargetState};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.targets(None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Filter targets by type:
    ///     let response = client.targets(Some(TargetState::Active)).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn targets(&self, state: Option<TargetState>) -> Result<Targets, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/targets");

        let mut params = vec![];

        if let Some(s) = &state {
            params.push(("state", s.to_string()))
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve a list of rule groups of recording and alerting rules.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#rules)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error, RuleType};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.rules(None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Filter rules by type:
    ///     let response = client.rules(Some(RuleType::Alert)).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rules(&self, rule_type: Option<RuleType>) -> Result<Vec<RuleGroup>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/rules");

        let mut params = vec![];

        if let Some(s) = rule_type {
            params.push(("type", s.to_string()))
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| {
                res.as_object()
                    .unwrap()
                    .get("groups")
                    .ok_or(Error::MissingField(MissingFieldError("groups")))
                    .and_then(|d| {
                        serde_json::from_value(d.to_owned()).map_err(Error::ResponseParse)
                    })
            })
    }

    /// Retrieve a list of active alerts.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#alerts)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.alerts().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn alerts(&self) -> Result<Vec<Alert>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/alerts");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| {
                res.as_object()
                    .unwrap()
                    .get("alerts")
                    .ok_or(Error::MissingField(MissingFieldError("alerts")))
                    .and_then(|d| {
                        serde_json::from_value(d.to_owned()).map_err(Error::ResponseParse)
                    })
            })
    }

    /// Retrieve a list of flags that Prometheus was configured with.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#flags)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.flags().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn flags(&self) -> Result<HashMap<String, String>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/status/flags");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve Prometheus server build information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#build-information)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.build_information().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn build_information(&self) -> Result<BuildInformation, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/status/buildinfo");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve Prometheus server runtime information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#runtime-information)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.runtime_information().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn runtime_information(&self) -> Result<RuntimeInformation, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/status/runtimeinfo");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve Prometheus TSDB statistics.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#tsdb-stats)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.tsdb_statistics().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn tsdb_statistics(&self) -> Result<TsdbStatistics, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/status/tsdb");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve WAL replay statistics.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#wal-replay-stats)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.wal_replay_statistics().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn wal_replay_statistics(&self) -> Result<WalReplayStatistics, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/status/walreplay");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Query the current state of alertmanager discovery.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#alertmanagers)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.alertmanagers().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn alertmanagers(&self) -> Result<Alertmanagers, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/alertmanagers");

        self.send(url, None, HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve metadata about metrics that are currently scraped from targets, along with target information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-target-metadata)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error, Selector};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     // Retrieve metadata for a specific metric from all targets.
    ///     let response = client.target_metadata(Some("go_routines"), None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metric metadata from specific targets.
    ///     let s = Selector::new().eq("job", "prometheus");
    ///
    ///     let response = client.target_metadata(None, Some(&s), None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metadata for a specific metric from targets that match a specific label set.
    ///     let s = Selector::new().eq("job", "node");
    ///
    ///     let response = client.target_metadata(Some("node_cpu_seconds_total"), Some(&s), None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn target_metadata(
        &self,
        metric: Option<&str>,
        match_target: Option<&Selector<'_>>,
        limit: Option<usize>,
    ) -> Result<Vec<TargetMetadata>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/targets/metadata");

        let mut params = vec![];

        if let Some(m) = metric {
            params.push(("metric", m.to_string()))
        }

        if let Some(m) = match_target {
            params.push(("match_target", m.to_string()))
        }

        if let Some(l) = &limit {
            params.push(("limit", l.to_string()))
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }

    /// Retrieve metadata about metrics that are currently scraped from targets.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-metric-metadata)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::default();
    ///
    ///     // Retrieve metadata for a all metrics.
    ///     let response = client.metric_metadata(None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Limit the number of returned metrics
    ///     let response = client.metric_metadata(None, Some(10)).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metadata of a specific metric.
    ///     let response = client.metric_metadata(Some("go_routines"), None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn metric_metadata(
        &self,
        metric: Option<&str>,
        limit: Option<usize>,
    ) -> Result<HashMap<String, Vec<MetricMetadata>>, Error> {
        let url = build_final_url(self.base_url.clone(), "api/v1/metadata");

        let mut params = vec![];

        if let Some(m) = &metric {
            params.push(("metric", m.to_string()))
        }

        if let Some(l) = &limit {
            params.push(("limit", l.to_string()))
        }

        self.send(url, Some(params), HttpMethod::GET)
            .await
            .and_then(check_api_response)
            .and_then(move |res| serde_json::from_value(res).map_err(Error::ResponseParse))
    }
}

// Convert the response object to an intermediary map, check the JSON's status field
// and map potential errors (if any) to a proper error type. Else return the map.
fn check_api_response(response: ApiResponse) -> Result<serde_json::Value, Error> {
    match response.status {
        ApiResponseStatus::Success => {
            let data = response
                .data
                .ok_or(Error::MissingField(MissingFieldError("data")))?;
            Ok(data)
        }
        ApiResponseStatus::Error => {
            let kind = response
                .error_type
                .ok_or(Error::MissingField(MissingFieldError("errorType")))?;

            let message = response
                .error
                .ok_or(Error::MissingField(MissingFieldError("error")))?;

            Err(Error::ApiError(ApiError { kind, message }))
        }
    }
}

// Parses the API response from a map to a Response enum that
// encapsulates a result type of "vector", "matrix", or "scalar".
fn convert_query_response(response: serde_json::Value) -> Result<QueryResult, Error> {
    let result: IntermediateQueryResult =
        serde_json::from_value(response).map_err(Error::ResponseParse)?;

    let data = match result.kind {
        QueryResultType::Vector => {
            let vector: Vec<InstantVector> =
                serde_json::from_value(result.data).map_err(Error::ResponseParse)?;
            Data::Vector(vector)
        }
        QueryResultType::Matrix => {
            let matrix: Vec<RangeVector> =
                serde_json::from_value(result.data).map_err(Error::ResponseParse)?;
            Data::Matrix(matrix)
        }
        QueryResultType::Scalar => {
            let sample: Sample =
                serde_json::from_value(result.data).map_err(Error::ResponseParse)?;
            Data::Scalar(sample)
        }
    };

    Ok(QueryResult {
        data,
        stats: result.stats,
    })
}
