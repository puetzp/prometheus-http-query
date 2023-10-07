use crate::error::{ClientError, Error};
use crate::response::*;
use crate::selector::Selector;
use crate::util::{self, build_final_url, RuleType, TargetState, ToBaseUrl};
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::Method as HttpMethod;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use url::Url;

/// A builder object used to set some query parameters in the context
/// of an instant query before sending the query on its way.
#[derive(Clone)]
pub struct InstantQueryBuilder {
    client: Client,
    params: Vec<(&'static str, String)>,
    headers: Option<HeaderMap<HeaderValue>>,
}

impl InstantQueryBuilder {
    /// Set the evaluation timestamp (Unix timestamp in seconds, e.g. 1659182624).
    /// If this is not set the evaluation timestamp will default to the current Prometheus
    /// server time.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    pub fn at(mut self, time: i64) -> Self {
        self.params.push(("time", time.to_string()));
        self
    }

    /// Set the evaluation timeout (milliseconds, e.g. 1000).
    /// If this is not set the timeout will default to the value of the "-query.timeout" flag of the Prometheus server.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.params.push(("timeout", format!("{}ms", timeout)));
        self
    }

    /// Instruct Prometheus to compile query statistics as part of the API response.
    pub fn stats(mut self) -> Self {
        self.params.push(("stats", String::from("all")));
        self
    }

    /// Include an additional header to the request.
    pub fn header<K: IntoHeaderName, T: Into<HeaderValue>>(mut self, name: K, value: T) -> Self {
        self.headers
            .get_or_insert_with(Default::default)
            .append(name, value.into());
        self
    }

    /// Execute the instant query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<PromqlResult, Error> {
        self.client
            .send("api/v1/query", &self.params, HttpMethod::GET, self.headers)
            .await
            .and_then(map_api_response)
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<PromqlResult, Error> {
        self.client
            .send("api/v1/query", &self.params, HttpMethod::POST, self.headers)
            .await
            .and_then(map_api_response)
    }
}

/// A builder object used to set some query parameters in the context
/// of a range query before sending the query on its way.
#[derive(Clone)]
pub struct RangeQueryBuilder {
    client: Client,
    params: Vec<(&'static str, String)>,
    headers: Option<HeaderMap<HeaderValue>>,
}

impl RangeQueryBuilder {
    /// Set the evaluation timeout (milliseconds, e.g. 1000).
    /// If this is not set the timeout will default to the value of the "-query.timeout" flag of the Prometheus server.
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#range-queries)
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.params.push(("timeout", format!("{}ms", timeout)));
        self
    }

    /// Instruct Prometheus to compile query statistics as part of the API response.
    pub fn stats(mut self) -> Self {
        self.params.push(("stats", String::from("all")));
        self
    }

    /// Include an additional header to the request.
    pub fn header<K: IntoHeaderName, T: Into<HeaderValue>>(mut self, name: K, value: T) -> Self {
        self.headers
            .get_or_insert_with(Default::default)
            .append(name, value.into());
        self
    }

    /// Execute the range query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<PromqlResult, Error> {
        self.client
            .send(
                "api/v1/query_range",
                &self.params,
                HttpMethod::GET,
                self.headers,
            )
            .await
            .and_then(map_api_response)
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<PromqlResult, Error> {
        self.client
            .send(
                "api/v1/query_range",
                &self.params,
                HttpMethod::POST,
                self.headers,
            )
            .await
            .and_then(map_api_response)
    }
}

/// A client used to execute queries. It uses a [`reqwest::Client`] internally
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

    /// Create a [`Client`] from a custom base URL. Note that the API-specific
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

    /// Create a [`Client`] from a custom base URL. Note that the API-specific
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
    /// Return a reference to the wrapped [`reqwest::Client`], i.e. to
    /// use it for other requests unrelated to the Prometheus API.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     // An amittedly bad example, but that is not the point.
    ///     let response = client
    ///         .inner()
    ///         .head("http://127.0.0.1:9090")
    ///         .send()
    ///         .await?;
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

    /// Create a Client from a custom [`reqwest::Client`] and URL.
    /// This way you can account for all extra parameters (e.g. x509 authentication)
    /// that may be needed to connect to Prometheus or an intermediate proxy,
    /// by building it into the [`reqwest::Client`].
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// fn main() -> Result<(), anyhow::Error> {
    ///     let client = {
    ///         let c = reqwest::Client::builder()
    ///             .no_proxy()
    ///             .build()?;
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

    /// Build and send the final HTTP request. Parse the result as JSON if the
    /// `Content-Type` header indicates that the payload is JSON. Otherwise it is
    /// assumed that an intermediate proxy sends a plain text error.
    async fn send<S: Serialize, D: DeserializeOwned>(
        &self,
        path: &str,
        params: &S,
        method: HttpMethod,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<ApiResponse<D>, Error> {
        let url = build_final_url(self.base_url.clone(), path);

        let mut request = match method {
            HttpMethod::GET => self.client.get(url).query(params),
            HttpMethod::POST => self.client.post(url).form(params),
            _ => unreachable!(),
        };

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request.send().await.map_err(|source| {
            Error::Client(ClientError {
                message: "failed to send request to server",
                source: Some(source),
            })
        })?;

        let header = CONTENT_TYPE;

        if util::is_json(response.headers().get(header)) {
            response.json::<ApiResponse<D>>().await.map_err(|source| {
                Error::Client(ClientError {
                    message: "failed to parse JSON response from server",
                    source: Some(source),
                })
            })
        } else {
            Err(Error::Client(ClientError {
                message: "failed to parse response from server due to invalid media type",
                source: response.error_for_status().err(),
            }))
        }
    }

    /// Create an [`InstantQueryBuilder`] from a PromQL query allowing you to set some query parameters
    /// (e.g. evaluation timeout) before finally sending the instant query to the server.
    ///
    /// # Arguments
    /// * `query` - PromQL query to exeute
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.query("prometheus_http_request_total").get().await?;
    ///
    ///     assert!(response.data().as_vector().is_some());
    ///
    ///     // Or make a POST request.
    ///     let response = client.query("prometheus_http_request_total").post().await?;
    ///
    ///     assert!(response.data().as_vector().is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn query(&self, query: impl std::string::ToString) -> InstantQueryBuilder {
        InstantQueryBuilder {
            client: self.clone(),
            params: vec![("query", query.to_string())],
            headers: Default::default(),
        }
    }

    /// Create a [`RangeQueryBuilder`] from a PromQL query allowing you to set some query parameters
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
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     let q = "prometheus_http_requests_total";
    ///
    ///     let response = client.query_range(q, 1648373100, 1648373300, 10.0).get().await?;
    ///
    ///     assert!(response.data().as_matrix().is_some());
    ///
    ///     // Or make a POST request.
    ///     let response = client.query_range(q, 1648373100, 1648373300, 10.0).post().await?;
    ///
    ///     assert!(response.data().as_matrix().is_some());
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
            params: vec![
                ("query", query.to_string()),
                ("start", start.to_string()),
                ("end", end.to_string()),
                ("step", step.to_string()),
            ],
            headers: Default::default(),
        }
    }

    /// Find time series that match certain label sets ([`Selector`]s).
    ///
    /// # Arguments
    /// * `selectors` - Iterable container of [`Selector`]s that tells Prometheus which series to return. Must not be empty!
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#finding-series-by-label-matchers)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
    pub async fn series<'a, T, I>(
        &self,
        selectors: T,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<HashMap<String, String>>, Error>
    where
        T: IntoIterator,
        T::IntoIter: Iterator<Item = I>,
        I: Borrow<Selector<'a>>,
    {
        let mut params = vec![];

        if let Some(s) = start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = end {
            params.push(("end", e.to_string()));
        }

        let mut matchers: Vec<(&str, String)> = selectors
            .into_iter()
            .map(|s| ("match[]", s.borrow().to_string()))
            .collect();

        if matchers.is_empty() {
            return Err(Error::EmptySeriesSelector);
        }

        params.append(&mut matchers);

        self.send("api/v1/series", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve label names.
    ///
    /// # Arguments
    /// * `selectors` - Iterable container of [`Selector`]s that tells Prometheus to read label names only from certain time series that match one of these `Selector`s. Pass an empty argument (e.g. `&[]`) in order to retrieve all label names.
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#getting-label-names)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     // To retrieve a list of all labels:
    ///     let response = client.label_names(&[], None, None).await;
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
    ///     let response = client.label_names(&[s1, s2], None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn label_names<'a, T, I>(
        &self,
        selectors: T,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<String>, Error>
    where
        T: IntoIterator,
        T::IntoIter: Iterator<Item = I>,
        I: Borrow<Selector<'a>>,
    {
        let mut params = vec![];

        if let Some(s) = &start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = &end {
            params.push(("end", e.to_string()));
        }

        let mut matchers: Vec<(&str, String)> = selectors
            .into_iter()
            .map(|s| ("match[]", s.borrow().to_string()))
            .collect();

        params.append(&mut matchers);

        self.send("api/v1/labels", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve all label values for a specific label name.
    ///
    /// # Arguments
    /// * `label` - Name of the label to return all occuring label values for.
    /// * `selectors` - Iterable collection of [`Selector`]s that tells Prometheus to read the label values only from certain time series that match one of these `Selector`s. Pass an empty collection (e.g `&[]`) in order to retrieve all label values for the specified label name.
    /// * `start` - Start timestamp as Unix timestamp (seconds). Optional.
    /// * `end` - End timestamp as Unix timestamp (seconds). Optional.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-label-values)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     // To retrieve a list of all label values for a specific label name:
    ///     let response = client.label_values("job", &[], None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of label values of labels that appear in specific time series:
    ///     let s1 = Selector::new()
    ///         .regex_eq("instance", ".+");
    ///
    ///     let response = client.label_values("job", &[s1], None, None).await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn label_values<'a, T, I>(
        &self,
        label: &str,
        selectors: T,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Vec<String>, Error>
    where
        T: IntoIterator,
        T::IntoIter: Iterator<Item = I>,
        I: Borrow<Selector<'a>>,
    {
        let mut params = vec![];

        if let Some(s) = &start {
            params.push(("start", s.to_string()));
        }

        if let Some(e) = &end {
            params.push(("end", e.to_string()));
        }

        let mut matchers: Vec<(&str, String)> = selectors
            .into_iter()
            .map(|s| ("match[]", s.borrow().to_string()))
            .collect();

        params.append(&mut matchers);

        let path = format!("api/v1/label/{}/values", label);
        self.send(&path, &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Query the current state of target discovery.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#targets)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, TargetState};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        let mut params = vec![];

        if let Some(s) = &state {
            params.push(("state", s.to_string()))
        }

        self.send("api/v1/targets", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve a list of rule groups of recording and alerting rules.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#rules)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, RuleType};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        let mut params = vec![];

        if let Some(s) = rule_type {
            params.push(("type", s.to_string()))
        }

        self.send("api/v1/rules", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
            .map(|r: RuleGroups| r.groups)
    }

    /// Retrieve a list of active alerts.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#alerts)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/alerts", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
            .map(|r: Alerts| r.alerts)
    }

    /// Retrieve a list of flags that Prometheus was configured with.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#flags)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/status/flags", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve Prometheus server build information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#build-information)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/status/buildinfo", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve Prometheus server runtime information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#runtime-information)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/status/runtimeinfo", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve Prometheus TSDB statistics.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#tsdb-stats)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/status/tsdb", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve WAL replay statistics.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#wal-replay-stats)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/status/walreplay", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Query the current state of alertmanager discovery.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#alertmanagers)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        self.send("api/v1/alertmanagers", &(), HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve metadata about metrics that are currently scraped from targets, along with target information.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-target-metadata)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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

        self.send("api/v1/targets/metadata", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Retrieve metadata about metrics that are currently scraped from targets.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-metric-metadata)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
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
        let mut params = vec![];

        if let Some(m) = &metric {
            params.push(("metric", m.to_string()))
        }

        if let Some(l) = &limit {
            params.push(("limit", l.to_string()))
        }

        self.send("api/v1/metadata", &params, HttpMethod::GET, None)
            .await
            .and_then(map_api_response)
    }

    /// Check Prometheus server health.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/management_api/#health-check)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///     assert!(client.is_server_healthy().await?);
    ///     Ok(())
    /// }
    /// ```
    pub async fn is_server_healthy(&self) -> Result<bool, Error> {
        let url = build_final_url(self.base_url.clone(), "-/healthy");
        self.client
            .get(url)
            .send()
            .await
            .map_err(|source| {
                Error::Client(ClientError {
                    message: "failed to send request to health endpoint",
                    source: Some(source),
                })
            })?
            .error_for_status()
            .map_err(|source| {
                Error::Client(ClientError {
                    message: "request to health endpoint returned an error",
                    source: Some(source),
                })
            })
            .map(|_| true)
    }

    /// Check Prometheus server readiness.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/management_api/#readiness-check)
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///     assert!(client.is_server_ready().await?);
    ///     Ok(())
    /// }
    /// ```
    pub async fn is_server_ready(&self) -> Result<bool, Error> {
        let url = build_final_url(self.base_url.clone(), "-/ready");
        self.client
            .get(url)
            .send()
            .await
            .map_err(|source| {
                Error::Client(ClientError {
                    message: "failed to send request to readiness endpoint",
                    source: Some(source),
                })
            })?
            .error_for_status()
            .map_err(|source| {
                Error::Client(ClientError {
                    message: "request to readiness endpoint returned an error",
                    source: Some(source),
                })
            })
            .map(|_| true)
    }
}

// Map the API response object to a Result:
// Data is returned as is, errors within the response body are converted to
// this crate's error type.
#[inline]
fn map_api_response<D: DeserializeOwned>(response: ApiResponse<D>) -> Result<D, Error> {
    match response {
        ApiResponse::Success { data } => Ok(data),
        ApiResponse::Error(e) => Err(Error::Prometheus(e)),
    }
}
