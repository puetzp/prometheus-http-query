use crate::error::{ClientError, Error};
use crate::response::*;
use crate::selector::Selector;
use crate::util::{self, build_final_url, RuleKind, TargetState, ToBaseUrl};
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::Method as HttpMethod;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use url::Url;

/// Provides a builder to set some query parameters in the context
/// of an instant query before sending it to Prometheus.
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

    /// Include an additional parameter to the request.
    pub fn query(mut self, name: &'static str, value: impl ToString) -> Self {
        self.params.push((name, value.to_string()));
        self
    }

    /// Execute the instant query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<PromqlResult, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<PromqlResult, Error> {
        let response = self.post_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the instant query (using HTTP GET) and return the raw API response.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        self.client
            .send("api/v1/query", &self.params, HttpMethod::GET, self.headers)
            .await
    }

    /// Execute the instant query (using HTTP POST) and return the raw API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post_raw(self) -> Result<reqwest::Response, Error> {
        self.client
            .send("api/v1/query", &self.params, HttpMethod::POST, self.headers)
            .await
    }
}

/// Provides a builder to set some query parameters in the context
/// of a range query before sending it to Prometheus.
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

    /// Include an additional parameter to the request.
    pub fn query(mut self, name: &'static str, value: impl ToString) -> Self {
        self.params.push((name, value.to_string()));
        self
    }

    /// Execute the range query (using HTTP GET) and return the parsed API response.
    pub async fn get(self) -> Result<PromqlResult, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the instant query (using HTTP POST) and return the parsed API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post(self) -> Result<PromqlResult, Error> {
        let response = self.post_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the range query (using HTTP GET) and return the raw API response.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        self.client
            .send(
                "api/v1/query_range",
                &self.params,
                HttpMethod::GET,
                self.headers,
            )
            .await
    }

    /// Execute the instant query (using HTTP POST) and return the raw API response.
    /// Using a POST request is useful in the context of larger PromQL queries when
    /// the size of the final URL may break Prometheus' or an intermediate proxies' URL
    /// character limits.
    pub async fn post_raw(self) -> Result<reqwest::Response, Error> {
        self.client
            .send(
                "api/v1/query_range",
                &self.params,
                HttpMethod::POST,
                self.headers,
            )
            .await
    }
}

/// Provides methods to build a query to the rules endpoint and send it to Prometheus.
#[derive(Clone)]
pub struct RulesQueryBuilder {
    client: Client,
    kind: Option<RuleKind>,
    names: Vec<String>,
    groups: Vec<String>,
    files: Vec<String>,
}

/// Note that Prometheus combines all filters that have been set in the final request
/// and only returns rules that match all filters.<br>
/// See the official documentation for a thorough explanation on the filters that can
/// be set: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#rules).
impl RulesQueryBuilder {
    /// Set this to instruct Prometheus to only return a specific type of rule
    /// (either recording or alerting rules) instead of both. Calling this repeatedly
    /// will replace the current setting.
    pub fn kind(mut self, kind: RuleKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Pass rule names to instruct Prometheus to only return those rules whose
    /// names match one of them. This method can be called repeatedly and merge
    /// the names with those that have been set before.
    pub fn names<T>(mut self, names: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        self.names.extend(names.into_iter().map(|n| n.to_string()));
        self
    }

    /// Pass a rule name to instruct Prometheus to return rules that match this name.
    /// This method can be called repeatedly to extend the set of rule names that
    /// will be sent to Prometheus.
    pub fn name(mut self, name: impl std::fmt::Display) -> Self {
        self.names.push(name.to_string());
        self
    }

    /// Pass group names to instruct Prometheus to only return those rules that are
    /// part of one of these groups. This method can be called repeatedly and merge
    /// the group names with those that have been set before.
    pub fn groups<T>(mut self, groups: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        self.groups
            .extend(groups.into_iter().map(|g| g.to_string()));
        self
    }

    /// Pass a group name to instruct Prometheus to return rules that are part of this
    /// group. This method can be called repeatedly to extend the set of group names
    /// that will be sent to Prometheus.
    pub fn group(mut self, group: impl std::fmt::Display) -> Self {
        self.groups.push(group.to_string());
        self
    }

    /// Pass file names to instruct Prometheus to only return those rules that are
    /// defined in one of those files. This method can be called repeatedly and merge
    /// the file names with those that have been set before.
    pub fn files<T>(mut self, files: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        self.files.extend(files.into_iter().map(|f| f.to_string()));
        self
    }

    /// Pass a file name to instruct Prometheus to return rules that are defined in
    /// this file. This method can be called repeatedly to extend the set of file names
    /// that will be sent to Prometheus.
    pub fn file(mut self, file: impl std::fmt::Display) -> Self {
        self.files.push(file.to_string());
        self
    }

    /// Execute the rules query (using HTTP GET) and return the [`RuleGroup`]s sent
    /// by Prometheus.
    pub async fn get(self) -> Result<Vec<RuleGroup>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response)
            .await
            .map(|r: RuleGroups| r.groups)
    }

    /// Execute the rules query (using HTTP GET) and return the raw response sent
    /// by Prometheus.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(k) = self.kind {
            params.push(("type", k.to_query_param()))
        }

        for name in self.names {
            params.push(("rule_name[]", name))
        }

        for group in self.groups {
            params.push(("rule_group[]", group))
        }

        for file in self.files {
            params.push(("file[]", file))
        }

        self.client
            .send("api/v1/rules", &params, HttpMethod::GET, None)
            .await
    }
}

/// Provides methods to build a query to the target metadata endpoint and send it to Prometheus.
#[derive(Clone)]
pub struct TargetMetadataQueryBuilder<'a> {
    client: Client,
    match_target: Option<Selector<'a>>,
    metric: Option<String>,
    limit: Option<i32>,
}

/// Note that Prometheus combines all filters that have been set in the final request
/// and only returns target metadata that matches all filters.<br>
/// See the official documentation for a thorough explanation on the filters that can
/// be set: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-target-metadata).
impl<'a> TargetMetadataQueryBuilder<'a> {
    /// Pass a label selector to instruct Prometheus to filter targets by their label
    /// sets.
    /// Calling this repeatedly will replace the current label selector.
    pub fn match_target(mut self, selector: &'a Selector<'a>) -> Self {
        self.match_target = Some(selector.clone());
        self
    }

    /// Set this to only retrieve target metadata for this metric.
    /// Calling this repeatedly will replace the current metric name.
    pub fn metric(mut self, metric: impl std::fmt::Display) -> Self {
        self.metric = Some(metric.to_string());
        self
    }

    /// Limit the maximum number of targets to match.
    /// Calling this repeatedly will replace the current limit.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Execute the target metadata query (using HTTP GET) and return the collection of
    /// [`TargetMetadata`] sent by Prometheus.
    pub async fn get(self) -> Result<Vec<TargetMetadata>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the target metadata query (using HTTP GET) and return the raw response
    /// sent by Prometheus.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(metric) = self.metric {
            params.push(("metric", metric.to_string()))
        }

        if let Some(match_target) = self.match_target {
            params.push(("match_target", match_target.to_string()))
        }

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()))
        }

        self.client
            .send("api/v1/targets/metadata", &params, HttpMethod::GET, None)
            .await
    }
}

/// Provides methods to build a query to the metric metadata endpoint and send it to Prometheus.
#[derive(Clone)]
pub struct MetricMetadataQueryBuilder {
    client: Client,
    metric: Option<String>,
    limit: Option<i32>,
    limit_per_metric: Option<i32>,
}

/// Note that Prometheus combines all filters that have been set in the final request
/// and only returns metric metadata that matches all filters.<br>
/// See the official documentation for a thorough explanation on the filters that can
/// be set: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-metric-metadata).
impl MetricMetadataQueryBuilder {
    /// Instruct Prometheus to filter metadata by this metric name.
    /// Calling this repeatedly will replace the current setting.
    pub fn metric(mut self, metric: impl std::fmt::Display) -> Self {
        self.metric = Some(metric.to_string());
        self
    }

    /// Limit the maximum number of metrics to return.
    /// Calling this repeatedly will replace the current limit.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Limit the maximum number of metadata to return per metric.
    /// Calling this repeatedly will replace the current limit.
    pub fn limit_per_metric(mut self, limit_per_metric: i32) -> Self {
        self.limit_per_metric = Some(limit_per_metric);
        self
    }

    /// Execute the metric metadata query (using HTTP GET) and return the collection of
    /// [`MetricMetadata`] sent by Prometheus.
    pub async fn get(self) -> Result<HashMap<String, Vec<MetricMetadata>>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the metric metadata query (using HTTP GET) and return the raw response
    /// sent by Prometheus.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(metric) = self.metric {
            params.push(("metric", metric.to_string()))
        }

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()))
        }

        if let Some(limit_per_metric) = self.limit_per_metric {
            params.push(("limit_per_metric", limit_per_metric.to_string()))
        }

        self.client
            .send("api/v1/metadata", &params, HttpMethod::GET, None)
            .await
    }
}

/// Provides methods to build a query to the series endpoint and send it to Prometheus.
#[derive(Clone)]
pub struct SeriesQueryBuilder {
    client: Client,
    selectors: Vec<(&'static str, String)>,
    start: Option<i64>,
    end: Option<i64>,
}

impl SeriesQueryBuilder {
    /// Limit the amount of metadata returned by setting a start time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Limit the amount of metadata returned by setting an end time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }

    /// Execute the series metadata query (using HTTP GET) and return a collection of
    /// matching time series sent by Prometheus.
    pub async fn get(self) -> Result<Vec<HashMap<String, String>>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the series metadata query (using HTTP GET) and return the raw response
    /// sent by Prometheus.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(start) = self.start {
            params.push(("start", start.to_string()));
        }

        if let Some(end) = self.end {
            params.push(("end", end.to_string()));
        }

        params.extend(self.selectors);

        self.client
            .send("api/v1/series", &params, HttpMethod::GET, None)
            .await
    }
}

/// Provides methods to build a query to retrieve label names from Prometheus.
#[derive(Clone)]
pub struct LabelNamesQueryBuilder {
    client: Client,
    selectors: Vec<(&'static str, String)>,
    start: Option<i64>,
    end: Option<i64>,
}

impl LabelNamesQueryBuilder {
    /// Set series selectors to filter the time series from wich Prometheus
    /// reads labels from.
    /// This can be called multiple times to merge the series selectors with
    /// those that have been set before.
    pub fn selectors<'a, T>(mut self, selectors: T) -> Self
    where
        T: IntoIterator,
        T::Item: Borrow<Selector<'a>>,
    {
        self.selectors.extend(
            selectors
                .into_iter()
                .map(|s| ("match[]", s.borrow().to_string())),
        );
        self
    }

    /// Limit the amount of metadata returned by setting a start time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Limit the amount of metadata returned by setting an end time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }

    /// Execute the query (using HTTP GET) and retrieve a collection of
    /// label names.
    pub async fn get(self) -> Result<Vec<String>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the query (using HTTP GET) and retrieve the raw response.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(start) = self.start {
            params.push(("start", start.to_string()));
        }

        if let Some(end) = self.end {
            params.push(("end", end.to_string()));
        }

        params.extend(self.selectors);

        self.client
            .send("api/v1/labels", &params, HttpMethod::GET, None)
            .await
    }
}

/// Provides methods to build a query to retrieve label values for a specific
/// label from Prometheus.
#[derive(Clone)]
pub struct LabelValuesQueryBuilder {
    client: Client,
    label: String,
    selectors: Vec<(&'static str, String)>,
    start: Option<i64>,
    end: Option<i64>,
}

impl LabelValuesQueryBuilder {
    /// Set series selectors to filter the time series from wich Prometheus
    /// reads label values from.
    /// This can be called multiple times to merge the series selectors with
    /// those that have been set before.
    pub fn selectors<'a, T>(mut self, selectors: T) -> Self
    where
        T: IntoIterator,
        T::Item: Borrow<Selector<'a>>,
    {
        self.selectors.extend(
            selectors
                .into_iter()
                .map(|s| ("match[]", s.borrow().to_string())),
        );
        self
    }

    /// Limit the amount of metadata returned by setting a start time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Limit the amount of metadata returned by setting an end time
    /// (UNIX timestamp in seconds).
    /// Calling this repeatedly will replace the current setting.
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }

    /// Execute the query (using HTTP GET) and retrieve a collection of
    /// label values for the given label name.
    pub async fn get(self) -> Result<Vec<String>, Error> {
        let response = self.get_raw().await?;
        Client::deserialize(response).await
    }

    /// Execute the query (using HTTP GET) and retrieve a collection of
    /// label values for the given label name.
    pub async fn get_raw(self) -> Result<reqwest::Response, Error> {
        let mut params = vec![];

        if let Some(start) = self.start {
            params.push(("start", start.to_string()));
        }

        if let Some(end) = self.end {
            params.push(("end", end.to_string()));
        }

        params.extend(self.selectors);

        let path = format!("api/v1/label/{}/values", self.label);
        self.client
            .send(&path, &params, HttpMethod::GET, None)
            .await
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
    async fn send<S: Serialize>(
        &self,
        path: &str,
        params: &S,
        method: HttpMethod,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<reqwest::Response, Error> {
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
        Ok(response)
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
    pub fn query(&self, query: impl std::fmt::Display) -> InstantQueryBuilder {
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
        query: impl std::fmt::Display,
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

    /// Create a [`SeriesQueryBuilder`] to apply filters to a series metadata
    /// query before sending it to Prometheus.
    ///
    /// # Arguments
    /// * `selectors` - Iterable container of [`Selector`]s that tells Prometheus which series to return. Must not be empty!
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
    ///     let response = client.series(&[s1, s2])?.get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn series<'a, T>(&self, selectors: T) -> Result<SeriesQueryBuilder, Error>
    where
        T: IntoIterator,
        T::Item: Borrow<Selector<'a>>,
    {
        let selectors: Vec<(&str, String)> = selectors
            .into_iter()
            .map(|s| ("match[]", s.borrow().to_string()))
            .collect();

        if selectors.is_empty() {
            Err(Error::EmptySeriesSelector)
        } else {
            Ok(SeriesQueryBuilder {
                client: self.clone(),
                selectors,
                start: None,
                end: None,
            })
        }
    }

    /// Create a [`LabelNamesQueryBuilder`] to apply filters to a query for the label
    /// names endpoint before sending it to Prometheus.
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
    ///     let response = client.label_names().get().await;
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
    ///     let response = client.label_names().selectors(&[s1, s2]).get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn label_names(&self) -> LabelNamesQueryBuilder {
        LabelNamesQueryBuilder {
            client: self.clone(),
            selectors: vec![],
            start: None,
            end: None,
        }
    }

    /// Create a [`LabelValuesQueryBuilder`] to apply filters to a query for the label
    /// values endpoint before sending it to Prometheus.
    ///
    /// # Arguments
    /// * `label` - Name of the label to return all occuring label values for.
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
    ///     let response = client.label_values("job").get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of label values of labels that appear in specific time series:
    ///     let s1 = Selector::new()
    ///         .regex_eq("instance", ".+");
    ///
    ///     let response = client.label_values("job").selectors(&[s1]).get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn label_values(&self, label: impl std::fmt::Display) -> LabelValuesQueryBuilder {
        LabelValuesQueryBuilder {
            client: self.clone(),
            label: label.to_string(),
            selectors: vec![],
            start: None,
            end: None,
        }
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

        let response = self
            .send("api/v1/targets", &params, HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
    }

    /// Create a [`RulesQueryBuilder`] to apply filters to the rules query before
    /// sending it to Prometheus.
    ///
    /// See also: [Prometheus API documentation](https://prometheus.io/docs/prometheus/latest/querying/api/#rules)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, RuleKind};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let client = Client::default();
    ///
    ///     let response = client.rules().get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Filter rules by type:
    ///     let response = client.rules().kind(RuleKind::Alerting).get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn rules(&self) -> RulesQueryBuilder {
        RulesQueryBuilder {
            client: self.clone(),
            kind: None,
            names: vec![],
            groups: vec![],
            files: vec![],
        }
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
        let response = self
            .send("api/v1/alerts", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response)
            .await
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
        let response = self
            .send("api/v1/status/flags", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
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
        let response = self
            .send("api/v1/status/buildinfo", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
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
        let response = self
            .send("api/v1/status/runtimeinfo", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
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
        let response = self
            .send("api/v1/status/tsdb", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
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
        let response = self
            .send("api/v1/status/walreplay", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
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
        let response = self
            .send("api/v1/alertmanagers", &(), HttpMethod::GET, None)
            .await?;
        Client::deserialize(response).await
    }

    /// Create a [`TargetMetadataQueryBuilder`] to apply filters to a target metadata
    /// query before sending it to Prometheus.
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
    ///     let response = client.target_metadata().metric("go_goroutines").get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metric metadata from specific targets.
    ///     let s = Selector::new().eq("job", "prometheus");
    ///
    ///     let response = client.target_metadata().match_target(&s).get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metadata for a specific metric from targets that match a specific label set.
    ///     let s = Selector::new().eq("job", "node");
    ///
    ///     let response = client.target_metadata()
    ///         .metric("node_cpu_seconds_total")
    ///         .match_target(&s)
    ///         .get()
    ///         .await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn target_metadata<'a>(&self) -> TargetMetadataQueryBuilder<'a> {
        TargetMetadataQueryBuilder {
            client: self.clone(),
            match_target: None,
            metric: None,
            limit: None,
        }
    }

    /// Create a [`MetricMetadataQueryBuilder`] to apply filters to a metric metadata
    /// query before sending it to Prometheus.
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
    ///     let response = client.metric_metadata().get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Limit the number of returned metrics.
    ///     let response = client.metric_metadata().limit(100).get().await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Retrieve metadata for a specific metric but with a per-metric
    ///     // metadata limit.
    ///     let response = client.metric_metadata()
    ///         .metric("go_goroutines")
    ///         .limit_per_metric(5)
    ///         .get()
    ///         .await;
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn metric_metadata(&self) -> MetricMetadataQueryBuilder {
        MetricMetadataQueryBuilder {
            client: self.clone(),
            metric: None,
            limit: None,
            limit_per_metric: None,
        }
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

    // Deserialize the raw reqwest response returned from the Prometheus server into a type `D` that implements serde's `Deserialize` trait.
    //
    // Internally, the response is deserialized into the [`ApiResponse`] type first.
    // On success, the data is returned as is. On failure, the error is mapped to the appropriate [`Error`] type.
    async fn deserialize<D: DeserializeOwned>(response: reqwest::Response) -> Result<D, Error> {
        let header = CONTENT_TYPE;
        if !util::is_json(response.headers().get(header)) {
            return Err(Error::Client(ClientError {
                message: "failed to parse response from server due to invalid media type",
                source: response.error_for_status().err(),
            }));
        }
        let response = response.json::<ApiResponse<D>>().await.map_err(|source| {
            Error::Client(ClientError {
                message: "failed to parse JSON response from server",
                source: Some(source),
            })
        })?;
        match response {
            ApiResponse::Success { data } => Ok(data),
            ApiResponse::Error(e) => Err(Error::Prometheus(e)),
        }
    }
}
