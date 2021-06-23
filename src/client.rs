use crate::error::{
    Error, InvalidFunctionArgument, ResponseError, UnknownResponseStatus,
    UnsupportedResponseDataType,
};
use crate::response::*;
use crate::selector::Selector;
use crate::util::{validate_duration, RuleType, TargetState};
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
        let url = format!("{}/query", self.base_url);

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

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response)
            .await
            .and_then(convert_query_response)
    }

    pub async fn query_range(
        &self,
        vector: impl std::fmt::Display,
        start: i64,
        end: i64,
        step: &str,
        timeout: Option<&str>,
    ) -> Result<Response, Error> {
        let url = format!("{}/query_range", self.base_url);

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

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response)
            .await
            .and_then(convert_query_response)
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
        let url = format!("{}/series", self.base_url);

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

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let data = r["data"].as_array().unwrap();

            let mut result = vec![];

            for datum in data {
                let metric: HashMap<String, String> =
                    serde_json::from_value(datum.to_owned()).unwrap();
                result.push(metric);
            }

            Ok(Response::Series(result))
        })
    }

    /// Retrieve all label names (or use [Selector]s to select time series to read label names from).
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Selector, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     // To retrieve a list of all labels:
    ///     let response = tokio_test::block_on( async { client.labels(None, None, None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of labels that appear in specific time series, use Selectors:
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
    ///     Ok(())
    /// }
    /// ```
    pub async fn labels(
        &self,
        selectors: Option<Vec<Selector<'_>>>,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<Response, Error> {
        let url = format!("{}/labels", self.base_url);

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

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let data = r["data"].as_array().unwrap();

            let mut result = vec![];

            for datum in data {
                result.push(datum.as_str().unwrap().to_owned());
            }

            Ok(Response::LabelNames(result))
        })
    }

    /// Retrieve all label values for a label name (or use [Selector]s to select the time series to read label values from)
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Selector, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     // To retrieve a list of all label values for a specific label name:
    ///     let response = tokio_test::block_on( async { client.label_values("job", None, None, None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // To retrieve a list of label values of labels in specific time series instead:
    ///     let s1 = Selector::new()
    ///         .regex_match("instance", ".+");
    ///
    ///     let set = Some(vec![s1]);
    ///
    ///     let response = tokio_test::block_on( async { client.label_values("job", set, None, None).await });
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
    ) -> Result<Response, Error> {
        let url = format!("{}/label/{}/values", self.base_url, label);

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

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let data = r["data"].as_array().unwrap();

            let mut result = vec![];

            for datum in data {
                result.push(datum.as_str().unwrap().to_owned());
            }

            Ok(Response::LabelValues(result))
        })
    }

    /// Query the current state of target discovery.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Error, TargetState};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let response = tokio_test::block_on( async { client.targets(None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Filter targets by type:
    ///     let response = tokio_test::block_on( async { client.targets(Some(TargetState::Active)).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn targets(&self, state: Option<TargetState>) -> Result<Response, Error> {
        let url = format!("{}/targets", self.base_url);

        let mut params = vec![];

        let state = state.map(|s| s.to_string());

        if let Some(s) = &state {
            params.push(("state", s.as_str()))
        }

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let raw_targets = r["data"].to_owned();
            let targets: Targets = serde_json::from_value(raw_targets).unwrap();
            Ok(Response::Targets(targets))
        })
    }

    /// Retrieve a list of rule groups of recording and alerting rules.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Error, RuleType};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let response = tokio_test::block_on( async { client.rules(None).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     // Filter rules by type:
    ///     let response = tokio_test::block_on( async { client.rules(Some(RuleType::Alert)).await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rules(&self, rule_type: Option<RuleType>) -> Result<Response, Error> {
        let url = format!("{}/rules", self.base_url);

        let mut params = vec![];

        let rule_type = rule_type.map(|s| s.to_string());

        if let Some(s) = &rule_type {
            params.push(("type", s.as_str()))
        }

        let response = self
            .client
            .get(&url)
            .query(params.as_slice())
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let groups = r["data"].as_object().unwrap()["groups"]
                .as_array()
                .unwrap()
                .to_owned();

            let mut result = vec![];

            for group in groups {
                let g: RuleGroup = serde_json::from_value(group).unwrap();
                result.push(g);
            }

            Ok(Response::Rules(result))
        })
    }

    /// Retrieve a list of active alerts.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Scheme, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let client = Client::new(Scheme::Http, "localhost", 9090);
    ///
    ///     let response = tokio_test::block_on( async { client.alerts().await });
    ///
    ///     assert!(response.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn alerts(&self) -> Result<Response, Error> {
        let url = format!("{}/alerts", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(Error::Reqwest)?
            .error_for_status()
            .map_err(Error::Reqwest)?;

        check_response(response).await.and_then(move |r| {
            let alerts = r["data"].as_object().unwrap()["alerts"]
                .as_array()
                .unwrap()
                .to_owned();

            let mut result = vec![];

            for alert in alerts {
                let a: Alert = serde_json::from_value(alert).unwrap();
                result.push(a);
            }

            Ok(Response::Alerts(result))
        })
    }
}

// Convert the response object to an intermediary map, check the JSON's status field
// and map potential errors (if any) to a proper error type. Else return the map.
async fn check_response(
    response: reqwest::Response,
) -> Result<HashMap<String, serde_json::Value>, Error> {
    let map = response
        .json::<HashMap<String, serde_json::Value>>()
        .await
        .map_err(Error::Reqwest)?;

    let status = map["status"].as_str().unwrap();

    match status {
        "success" => Ok(map),
        "error" => Err(Error::ResponseError(ResponseError {
            kind: map["errorType"].as_str().unwrap().to_string(),
            message: map["error"].as_str().unwrap().to_string(),
        })),
        _ => Err(Error::UnknownResponseStatus(UnknownResponseStatus(
            status.to_string(),
        ))),
    }
}

// Parses the API response from a map to a Response enum that
// encapsulates a vector of samples of type "vector" or "matrix"
fn convert_query_response(response: HashMap<String, serde_json::Value>) -> Result<Response, Error> {
    let data_obj = response["data"].as_object().unwrap();
    let data_type = data_obj["resultType"].as_str().unwrap();
    let data = data_obj["result"].as_array().unwrap().to_owned();

    match data_type {
        "vector" => {
            let mut result: Vec<Vector> = vec![];

            for datum in data {
                let vector: Vector = serde_json::from_value(datum).unwrap();
                result.push(vector);
            }

            Ok(Response::Vector(result))
        }
        "matrix" => {
            let mut result: Vec<Matrix> = vec![];

            for datum in data {
                let matrix: Matrix = serde_json::from_value(datum).unwrap();
                result.push(matrix);
            }

            Ok(Response::Matrix(result))
        }
        _ => Err(Error::UnsupportedResponseDataType(
            UnsupportedResponseDataType(data_type.to_string()),
        )),
    }
}
