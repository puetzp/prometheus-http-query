use crate::error::Error;
use crate::response::instant::InstantQueryResponse;
use crate::response::range::RangeQueryResponse;
use crate::util::validate_timestamp;

/// A helper enum that is passed to the `Client::new` function in
/// order to avoid errors on unsupported connection schemes.
pub enum Scheme {
    Http,
    Https,
}

impl Scheme {
    fn as_str(&self) -> &str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
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
    /// let client = Client::new(Scheme::Http, "localhost", 9090);
    /// ```
    pub fn new(scheme: Scheme, host: &str, port: u16) -> Self {
        Client {
            base_url: format!("{}://{}:{}/api/v1", scheme.as_str(), host, port),
            ..Default::default()
        }
    }

    pub async fn execute_instant(
        &self,
        query: String,
        time: Option<&str>,
        timeout: Option<&str>,
    ) -> Result<InstantQueryResponse, Error> {
        if let Some(t) = time {
            if !validate_timestamp(t) {
                return Err(Error::InvalidTimestamp);
            }
        }

        let mut url = self.base_url.clone();

        url.push_str("/query");

        let mut params = vec![("query", query.as_str())];

        if let Some(t) = time {
            params.push(("time", t));
        }

        if let Some(t) = timeout {
            params.push(("timeout", t));
        }

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        match response.error_for_status() {
            Ok(res) => res
                .json::<InstantQueryResponse>()
                .await
                .map_err(Error::Reqwest),
            Err(err) => Err(Error::Reqwest(err)),
        }
    }

    pub async fn execute_range(
        &self,
        query: String,
        start: &str,
        end: &str,
        step: &str,
        timeout: Option<&str>,
    ) -> Result<InstantQueryResponse, Error> {
        if !validate_timestamp(start) {
            return Err(Error::InvalidTimestamp);
        }

        if !validate_timestamp(end) {
            return Err(Error::InvalidTimestamp);
        }

        let mut url = self.base_url.clone();

        url.push_str("/query");

        let mut params = vec![
            ("query", query.as_str()),
            ("start", start),
            ("end", end),
            ("step", step),
        ];

        if let Some(t) = timeout {
            params.push(("timeout", t));
        }

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        match response.error_for_status() {
            Ok(res) => res
                .json::<InstantQueryResponse>()
                .await
                .map_err(Error::Reqwest),
            Err(err) => Err(Error::Reqwest(err)),
        }
    }
}
