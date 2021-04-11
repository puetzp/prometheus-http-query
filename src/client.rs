use crate::query::*;
use crate::response::instant::*;
use crate::response::range::*;

/// A helper enum that is passed to the `Client::new` function in
/// order to avoid errors on unsupported connection schemes.
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl Scheme {
    fn as_str(&self) -> &str {
        match self {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
        }
    }
}

/// A client used to execute queries. It uses a `reqwest::Client` internally
/// that manages connections for us.
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
}

impl Default for Client {
    /// Create a Client that connects to a local Prometheus instance at port 9090.
    ///
    /// ```rust
    /// use prometheus_http_query::Client;
    ///
    /// let client: Client = Default::default();
    /// ```
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            base_url: String::from("http://127.0.0.1:9090/api/v1"),
        }
    }
}

impl Client {
    /// Create a Client that connects to a Prometheus instance at the
    /// given FQDN/domain and port, using either HTTP or HTTPS.
    ///
    /// Note that possible errors regarding domain name resolution or
    /// connection establishment will only be propagated from the underlying
    /// `reqwest::Client` when a query is executed.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme};
    ///
    /// let client = Client::new(Scheme::HTTP, "localhost", 9090);
    /// ```
    pub fn new(scheme: Scheme, host: &str, port: u16) -> Self {
        Client {
            base_url: format!("{}://{}:{}/api/v1", scheme.as_str(), host, port),
            ..Default::default()
        }
    }

    /// Execute an instant query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, InstantQuery};
    ///
    /// let client: Client = Default::default();
    /// let query = InstantQuery {
    ///     query: "up",
    ///     time: None,
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { client.instant(&query).await.unwrap() });
    /// assert!(!response.data.result.is_empty());
    /// ```
    pub async fn instant(
        &self,
        query: &InstantQuery<'_>,
    ) -> Result<InstantQueryResponse, reqwest::Error> {
        let mut url = self.base_url.clone();

        url.push_str("/query");

        let params = query.get_query_params();

        Ok(self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?
            .json::<InstantQueryResponse>()
            .await?)
    }

    /// Execute an instant query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, RangeQuery};
    ///
    /// let client: Client = Default::default();
    /// let query = RangeQuery {
    ///     query: "up",
    ///     start: "2021-04-09T11:30:00.000+02:00",
    ///     end: "2021-04-09T12:30:00.000+02:00",
    ///     step: "5m",
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { client.range(&query).await.unwrap() });
    /// assert!(!response.data.result.is_empty());
    /// ```
    pub async fn range(
        &self,
        query: &RangeQuery<'_>,
    ) -> Result<RangeQueryResponse, reqwest::Error> {
        let mut url = self.base_url.clone();

        url.push_str("/query_range");

        let params = query.get_query_params();

        Ok(self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?
            .json::<RangeQueryResponse>()
            .await?)
    }
}
