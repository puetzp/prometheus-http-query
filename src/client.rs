use crate::error::{
    Error, InvalidFunctionArgument, ResponseError, UnknownResponseStatus,
    UnsupportedResponseDataType,
};
use crate::response::*;
use crate::selector::Selector;
use crate::util::validate_duration;
use std::collections::HashMap;

/// A helper enum that is passed to the [Client::new] function in
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

/// A client used to execute queries. It uses a [reqwest::Client] internally
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
    /// [reqwest::Client] when a query is executed.
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

    /// Perform an instant query using a [crate::RangeVector] or [crate::InstantVector].
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, InstantVector, Selector, Aggregate, Error};
    /// use prometheus_http_query::aggregations::sum;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let v: InstantVector = Selector::new()
    ///         .metric("node_cpu_seconds_total")?
    ///         .try_into()?;
    ///
    ///     let s = sum(v, Some(Aggregate::By(&["cpu"])));
    ///
    ///     let response = tokio_test::block_on( async { client.query(s, None, None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        vector: impl std::fmt::Display,
        time: Option<i64>,
        timeout: Option<&str>,
    ) -> Result<Response, Error> {
        let mut url = self.base_url.clone();

        url.push_str("/query");

        let query = vector.to_string();
        let mut params = vec![("query", query.as_str())];

        let time = time.map(|t| t.to_string());

        if let Some(t) = &time {
            params.push(("time", t.as_str()));
        }

        if let Some(t) = timeout {
            validate_duration(t)?;
            params.push(("timeout", t));
        }

        let raw_response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        let mapped_response = match raw_response.error_for_status() {
            Ok(res) => res
                .json::<HashMap<String, serde_json::Value>>()
                .await
                .map_err(Error::Reqwest)?,
            Err(err) => return Err(Error::Reqwest(err)),
        };

        parse_query_response(mapped_response)
    }

    pub async fn query_range(
        &self,
        vector: impl std::fmt::Display,
        start: i64,
        end: i64,
        step: &str,
        timeout: Option<&str>,
    ) -> Result<Response, Error> {
        let mut url = self.base_url.clone();

        url.push_str("/query");

        validate_duration(step)?;

        let query = vector.to_string();
        let start = start.to_string();
        let end = end.to_string();

        let mut params = vec![
            ("query", query.as_str()),
            ("start", start.as_str()),
            ("end", end.as_str()),
            ("step", step),
        ];

        if let Some(t) = timeout {
            validate_duration(t)?;
            params.push(("timeout", t));
        }

        let raw_response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        let mapped_response = match raw_response.error_for_status() {
            Ok(res) => res
                .json::<HashMap<String, serde_json::Value>>()
                .await
                .map_err(Error::Reqwest)?,
            Err(err) => return Err(Error::Reqwest(err)),
        };

        parse_query_response(mapped_response)
    }

    /// Find time series by series selectors.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Selector, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let s1 = Selector::new()
    ///         .with("handler", "/api/v1/query");
    ///
    ///     let s2 = Selector::new()
    ///         .with("job", "node")
    ///         .regex_match("mode", ".+");
    ///
    ///     let set = vec![s1, s2];
    ///
    ///     let response = tokio_test::block_on( async { client.series(&set, None, None).await });
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
    ) -> Result<Response, Error> {
        let mut url = self.base_url.clone();

        url.push_str("/series");

        let mut params = vec![];

        let start = start.map(|t| t.to_string());

        if let Some(s) = &start {
            params.push(("start", s.as_str()));
        }

        let end = end.map(|t| t.to_string());

        if let Some(e) = &end {
            params.push(("end", e.as_str()));
        }

        if selectors.is_empty() {
            return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
                message: String::from("at least one match[] argument (Selector) must be provided in order to query the series endpoint")
            }));
        }

        let selectors: Vec<String> = selectors
            .iter()
            .map(|s| match s.to_string().as_str().split_once('}') {
                Some(split) => {
                    let mut s = split.0.to_owned();
                    s.push('}');
                    s
                }
                None => s.to_string(),
            })
            .collect();

        for selector in &selectors {
            params.push(("match[]", &selector));
        }

        let raw_response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        let mapped_response = match raw_response.error_for_status() {
            Ok(res) => res
                .json::<HashMap<String, serde_json::Value>>()
                .await
                .map_err(Error::Reqwest)?,
            Err(err) => return Err(Error::Reqwest(err)),
        };

        parse_series_response(mapped_response)
    }

    /// Find label names by series selectors.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Selector, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let s1 = Selector::new()
    ///         .with("handler", "/api/v1/query");
    ///
    ///     let s2 = Selector::new()
    ///         .with("job", "node")
    ///         .regex_match("mode", ".+");
    ///
    ///     let set = Some(vec![s1, s2]);
    ///
    ///     let response = tokio_test::block_on( async { client.labels(set, None, None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of all labels instead:
    ///     let response = tokio_test::block_on( async { client.labels(None, None, None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn labels(
        &self,
        selectors: Option<Vec<Selector<'_>>>,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Response, Error> {
        let mut url = self.base_url.clone();

        url.push_str("/labels");

        let mut params = vec![];

        let start = start.map(|t| t.to_string());

        if let Some(s) = &start {
            params.push(("start", s.as_str()));
        }

        let end = end.map(|t| t.to_string());

        if let Some(e) = &end {
            params.push(("end", e.as_str()));
        }

        let selectors: Option<Vec<String>> = selectors.map(|vec| {
            vec.iter()
                .map(|s| match s.to_string().as_str().split_once('}') {
                    Some(split) => {
                        let mut s = split.0.to_owned();
                        s.push('}');
                        s
                    }
                    None => s.to_string(),
                })
                .collect()
        });

        if let Some(ref selector_vec) = selectors {
            for selector in selector_vec {
                params.push(("match[]", &selector));
            }
        }

        let raw_response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        // NOTE: Can be changed to .map(async |resp| resp.json ...)
        // when async closures are stable.
        let mapped_response = match raw_response.error_for_status() {
            Ok(res) => res
                .json::<HashMap<String, serde_json::Value>>()
                .await
                .map_err(Error::Reqwest)?,
            Err(err) => return Err(Error::Reqwest(err)),
        };

        parse_labels_response(mapped_response)
    }
}

// Parses the API response from a loosely typed Hashmap to a Response that
// encapsulates a vector of Hashmaps that hold label names and values.
// "Value"s are rigorously "unwrapped" in the process as each of these
// is expected to be part of the JSON response.
fn parse_series_response(response: HashMap<String, serde_json::Value>) -> Result<Response, Error> {
    let status = response["status"].as_str().unwrap();

    match status {
        "success" => {
            let data = response["data"].as_array().unwrap();

            let mut result = vec![];

            for datum in data {
                result.push(parse_metric(datum.as_object().unwrap()));
            }

            Ok(Response::Series(result))
        }
        "error" => {
            return Err(Error::ResponseError(ResponseError {
                kind: response["errorType"].as_str().unwrap().to_string(),
                message: response["error"].as_str().unwrap().to_string(),
            }))
        }
        _ => {
            return Err(Error::UnknownResponseStatus(UnknownResponseStatus(
                status.to_string(),
            )))
        }
    }
}

// Parses the API response from a loosely typed Hashmap to a Response that
// encapsulates a vector of label names.
// "Value"s are rigorously "unwrapped" in the process as each of these
// is expected to be part of the JSON response.
fn parse_labels_response(response: HashMap<String, serde_json::Value>) -> Result<Response, Error> {
    let status = response["status"].as_str().unwrap();

    match status {
        "success" => {
            let data = response["data"].as_array().unwrap();

            let mut result = vec![];

            for datum in data {
                result.push(datum.as_str().unwrap().to_owned());
            }

            Ok(Response::Labels(result))
        }
        "error" => {
            return Err(Error::ResponseError(ResponseError {
                kind: response["errorType"].as_str().unwrap().to_string(),
                message: response["error"].as_str().unwrap().to_string(),
            }))
        }
        _ => {
            return Err(Error::UnknownResponseStatus(UnknownResponseStatus(
                status.to_string(),
            )))
        }
    }
}

// Parses the API response from a loosely typed Hashmap to a Response that
// encapsulates a vector of samples of type "vector" or "matrix"
// "Value"s are rigorously "unwrapped" in the process as each of these
// is expected to be part of the JSON response.
fn parse_query_response(response: HashMap<String, serde_json::Value>) -> Result<Response, Error> {
    let status = response["status"].as_str().unwrap();

    match status {
        "success" => {
            let data_obj = response["data"].as_object().unwrap();
            let data_type = data_obj["resultType"].as_str().unwrap();
            let data = data_obj["result"].as_array().unwrap();

            match data_type {
                "vector" => {
                    let mut result: Vec<Vector> = vec![];

                    for datum in data {
                        let metric = parse_metric(datum["metric"].as_object().unwrap());

                        let raw_value = datum["value"].as_array().unwrap();

                        let sample = Sample {
                            timestamp: raw_value[0].as_f64().unwrap(),
                            value: raw_value[1].as_str().unwrap().to_string(),
                        };

                        result.push(Vector { metric, sample });
                    }

                    Ok(Response::Vector(result))
                }
                "matrix" => {
                    let mut result: Vec<Matrix> = vec![];

                    for datum in data {
                        let metric = parse_metric(datum["metric"].as_object().unwrap());

                        let mut samples: Vec<Sample> = vec![];

                        for sample in datum["values"].as_array().unwrap() {
                            samples.push(Sample {
                                timestamp: sample[0].as_f64().unwrap(),
                                value: sample[1].as_str().unwrap().to_string(),
                            });
                        }

                        result.push(Matrix { metric, samples });
                    }

                    Ok(Response::Matrix(result))
                }
                _ => {
                    return Err(Error::UnsupportedResponseDataType(
                        UnsupportedResponseDataType(data_type.to_string()),
                    ))
                }
            }
        }
        "error" => {
            return Err(Error::ResponseError(ResponseError {
                kind: response["errorType"].as_str().unwrap().to_string(),
                message: response["error"].as_str().unwrap().to_string(),
            }))
        }
        _ => {
            return Err(Error::UnknownResponseStatus(UnknownResponseStatus(
                status.to_string(),
            )))
        }
    }
}

fn parse_metric(metric: &serde_json::Map<String, serde_json::Value>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for item in metric {
        let key = item.0.to_string();
        let val = item.1.as_str().unwrap().to_string();
        result.insert(key, val);
    }

    result
}
