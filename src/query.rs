use crate::client::Client;
use crate::response::instant::InstantQueryResponse;
use crate::response::range::RangeQueryResponse;
use async_trait::async_trait;

#[async_trait]
pub trait Query<T> {
    fn get_query_params(&self) -> Vec<(&str, &str)>;
    async fn execute(&self, client: &Client) -> Result<T, reqwest::Error>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

#[async_trait]
impl<'a> Query<InstantQueryResponse> for InstantQuery<'a> {
    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("query", self.query)];

        if let Some(t) = &self.time {
            params.push(("time", t));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout", t));
        }

        params
    }

    /// Execute an instant query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, InstantQuery};
    ///
    /// let client: Client = Default::default();
    /// let query = InstantQuery {
    ///     query: "up",
    ///     time: None,
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(!response.data.result.is_empty());
    /// ```
    async fn execute(&self, client: &Client) -> Result<InstantQueryResponse, reqwest::Error> {
        let mut url = client.base_url.clone();

        url.push_str("/query");

        let params = self.get_query_params();

        let response = client
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?;

        match response.error_for_status() {
            Ok(res) => res.json::<InstantQueryResponse>().await,
            Err(err) => Err(err),
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

    /// Execute an instant query.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Query, RangeQuery};
    ///
    /// let client: Client = Default::default();
    /// let query = RangeQuery {
    ///     query: "up",
    ///     start: "2021-04-09T11:30:00.000+02:00",
    ///     end: "2021-04-09T12:30:00.000+02:00",
    ///     step: "5m",
    ///     timeout: None,
    /// };
    /// let response = tokio_test::block_on( async { query.execute(&client).await.unwrap() });
    /// assert!(!response.data.result.is_empty());
    /// ```
    async fn execute(&self, client: &Client) -> Result<RangeQueryResponse, reqwest::Error> {
        let mut url = client.base_url.clone();

        url.push_str("/query_range");

        let params = self.get_query_params();

        let response = client
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?;

        match response.error_for_status() {
            Ok(res) => res.json::<RangeQueryResponse>().await,
            Err(err) => Err(err),
        }
    }
}
