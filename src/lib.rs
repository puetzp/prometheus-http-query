//! The goal of this crate is to provide a query interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/) and leverage Rust's type system in the process. Thus mistakes while building a query can be caught at compile-time (or at least before actually sending the query to Prometheus).
//!
//! Most of the features of PromQL are mirrored in this library. Queries are gradually built from time series selectors, aggregations
//! and functions and then passed to an HTTP client to execute. Methods to retrieve various kinds of metadata and configuration are also implemented.
//!
//! The [Client] uses as [reqwest::Client] as HTTP client internally as you will see in the usage section. Thus its features and limitations also apply to this library.
//!
//! # Usage
//!
//! ## Initialize a client
//!
//! The [Client] can be constructed in various ways depending on your need to add customizations.
//!
//! ```rust
//! use prometheus_http_query::Client;
//! use std::str::FromStr;
//!
//! // In the most general case the default implementation is used to create the client.
//! // Requests will be sent to "http://127.0.0.1:9090 (the default listen address and port of the Prometheus server).
//! let client = Client::default();
//!
//! // Provide an alternative URL if you need to. The URL will be checked for correctness.
//! use std::convert::TryFrom;
//! let client = Client::try_from("https://prometheus.example.com").unwrap();
//!
//! // The greatest flexibility is offered by initializing a reqwest::Client first with
//! // all needed customizations and passing it along.
//! let client = {
//!     let c = reqwest::Client::builder().no_proxy().build().unwrap();
//!     Client::from(c, "https://prometheus.example.com").unwrap();
//! };
//! ```
//!
//! ## Construct PromQL queries
//!
//! Gradually build PromQL expressions using [Selector], turn it into a [RangeVector] or [InstantVector],
//! apply additional [aggregations] or [functions] on them and evaluate the final expression at an instant ([Client::query])
//! or a range of time ([Client::query_range]).
//!
//! ```rust
//! use prometheus_http_query::{Aggregate, Client, Error, InstantVector, Selector};
//! use prometheus_http_query::aggregations::{sum, topk};
//! use std::convert::TryInto;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let vector: InstantVector = Selector::new()
//!         .metric("prometheus_http_requests_total")
//!         .try_into()?;
//!
//!     let q = topk(vector, Some(Aggregate::By(&["code"])), 5);
//!
//!     let response = client.query(q, None, None).await?;
//!
//!     assert!(response.as_instant().is_some());
//!
//!     let vector: InstantVector = Selector::new()
//!         .metric("prometheus_http_requests_total")
//!         .with("code", "200")
//!         .try_into()?;
//!
//!     let q = sum(vector, None);
//!
//!     let response = client.query(q, None, None).await?;
//!
//!     if let Some(result) = response.as_instant() {
//!         let first = result.get(0).unwrap();
//!         println!("Received a total of {} HTTP requests", first.sample().value());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Custom non-validated PromQL queries
//!
//! It is also possible to bypass every kind of validation by supplying
//! a custom query directly to the [InstantVector] / [RangeVector] types.
//!
//! ```rust
//! use prometheus_http_query::{Client, Error, RangeVector};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let q = r#"sum by(cpu) (rate(node_cpu_seconds_total{mode="user"}[5m]))"#;
//!
//!     let v = RangeVector(q.to_string());
//!
//!     let response = client.query(v, None, None).await?;
//!
//!     assert!(response.as_instant().is_some());
//!    
//!     Ok(())
//! }
//! ```
//!
//! ## Metadata queries
//!
//! Retrieve a list of time series that match a certain label set by providing one or more series [Selector]s.
//!
//! ```rust
//! use prometheus_http_query::{Client, Error, Selector};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let s1 = Selector::new()
//!         .with("handler", "/api/v1/query");
//!
//!     let s2 = Selector::new()
//!         .with("job", "node")
//!         .regex_match("mode", ".+");
//!
//!     let set = vec![s1, s2];
//!
//!     let response = client.series(&set, None, None).await;
//!
//!     assert!(response.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Rules & Alerts
//!
//! Retrieve recording/alerting rules and active alerts.
//!
//! ```rust
//! use prometheus_http_query::{Client, Error, RuleType};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let response = client.rules(None).await;
//!
//!     assert!(response.is_ok());
//!
//!     // Only request alerting rules instead:
//!     let response = client.rules(Some(RuleType::Alert)).await;
//!
//!     assert!(response.is_ok());
//!
//!     // Request active alerts:
//!     let response = client.alerts().await;
//!
//!     assert!(response.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! These are a few examples. See [Client] for examples of other types of metadata queries.
//!
//! # Compatibility
//!
//! See the `README` for details on this matter.
//!
//! # Supported operations
//!
//! - [x] Building PromQL expressions using time series selectors, functions and operators (aggregation/binary/vector matching ...)
//! - [x] Evaluating expressions as instant queries
//! - [x] Evaluating expressions as range queries
//! - [x] Executing series metadata queries
//! - [x] Executing label metadata queries (names/values)
//! - [x] Retrieving target discovery status
//! - [x] Retrieve alerting + recording rules
//! - [x] Retrieve active alerts
//! - [x] Retrieve configured flags & values
//! - [x] Target metadata
//! - [x] Metric metadata
//! - [x] Alertmanager service discovery status
//! - [ ] Prometheus config
//! - [ ] Prometheus runtime & build information
//!
//! # Notes
//!
//! ## On parsing an error handling
//!
//! If the JSON response from the Prometheus HTTP API indicates an error (field `status` == `"error"`),
//! then the contents of both fields `errorType` and `error` are captured and then returned by the client
//! as a variant of the [Error] enum, just as any HTTP errors (non-200) that may indicate a problem
//! with the provided query string. Thus any syntax problems etc. that cannot be caught at compile time
//! or before executing the query will at least be propagated as returned by the HTTP API.
//!
//! ## On types
//!
//! This library uses two versions of instant vector and range vector types. [InstantVector] to build queries and be passed to [Client::query] or [Client::query_range] to execute. And [crate::response::InstantVector] as part of the result of these methods. The same applies to range vectors.
//!
//! # Limitations
//!
//! * Subqueries are not supported (only as custom query)
//! * PromQL functions that do not take a range / instant vector as an argument are not supported (only as custom query), e.g. pi()
//! * The [String](https://prometheus.io/docs/prometheus/latest/querying/api/#strings) result type is not supported
pub mod aggregations;
mod client;
mod error;
pub mod functions;
pub mod response;
mod selector;
mod util;
mod vector;
pub use self::client::Client;
pub use self::error::Error;
pub use self::selector::Selector;
pub use self::util::Aggregate;
pub use self::util::Group;
pub use self::util::Match;
pub use self::util::RuleType;
pub use self::util::TargetState;
pub use self::vector::InstantVector;
pub use self::vector::RangeVector;
