#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Default for Client {
    /// Create a Client that connects to localhost on port 8080 for requests.
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            base_url: String::from("http://127.0.0.1:9090/api/v1"),
        }
    }
}

impl Client {
    pub fn new(host: &str, port: u16, scheme: &str) -> Self {
        let mut base_url = scheme.to_owned();
        base_url.push_str("://");
        base_url.push_str(host);
        base_url.push(':');
        base_url.push_str(&port.to_string());
        base_url.push_str("/api/v1");

        Client {
            base_url: base_url,
            ..Default::default()
        }
    }

    pub async fn instant_query(&self, query: &str) -> String {
        let mut url = self.base_url.clone();
        url.push_str("/query");
        let result = self
            .client
            .get(&url)
            .query(&[("query", query)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        result
    }
}
