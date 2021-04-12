use crate::client::Client;
use crate::response::instant::InstantQueryResponse;
use crate::response::range::RangeQueryResponse;
use async_trait::async_trait;

#[async_trait]
pub trait Query {
    type Response;

    fn get_query_params(&self) -> Vec<(&str, &str)>;
    async fn execute(&self, client: &Client) -> Result<Self::Response, reqwest::Error>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

#[async_trait]
impl<'a> Query for InstantQuery<'a> {
    type Response = InstantQueryResponse;

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
    async fn execute(&self, client: &Client) -> Result<Self::Response, reqwest::Error> {
        let mut url = client.base_url.clone();

        url.push_str("/query");

        let params = self.get_query_params();

        Ok(client
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?
            .json::<InstantQueryResponse>()
            .await?)
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
impl<'a> Query for RangeQuery<'a> {
    type Response = RangeQueryResponse;

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
    async fn execute(&self, client: &Client) -> Result<Self::Response, reqwest::Error> {
        let mut url = client.base_url.clone();

        url.push_str("/query");

        let params = self.get_query_params();

        Ok(client
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await?
            .json::<RangeQueryResponse>()
            .await?)
    }
}
