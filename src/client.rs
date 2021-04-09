use crate::result::QueryResult;

pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Default for Client {
    /// Create a Client that connects to localhost on port 9090 for requests.
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            base_url: String::from("http://127.0.0.1:9090/api/v1"),
        }
    }
}

impl Client {
    pub fn new(host: &str, port: u16, scheme: &str) -> Self {
        Client {
            base_url: format!("{}://{}:{}/api/v1", scheme, host, port),
            ..Default::default()
        }
    }

    pub async fn instant_query(
        &self,
        query: &str,
    ) -> Result<QueryResult, Box<dyn std::error::Error>> {
        let mut url = self.base_url.clone();
        url.push_str("/query");

        let result = self
            .client
            .get(&url)
            .query(&[("query", query)])
            .send()
            .await?
            .json::<QueryResult>()
            .await?;

        Ok(result)
    }
}
