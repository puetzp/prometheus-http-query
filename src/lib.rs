//! The goal of this crate is to provide a query interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/) and leverage Rust's type system in the process. Thus mistakes while building a query can be caught at compile-time (or at least before actually sending the query to Prometheus).
//!
//! Most of the features of PromQL are mirrored in this library. Queries are gradually built from time series selectors, aggregations
//! and functions and then passed to an HTTP client to execute.
//!
//! Behind the scenes this library uses the [reqwest] crate as a HTTP client. Thus its features and limitations also
//! apply to this library.
//!
//! # Usage
//!
//! ## Construct PromQL queries
//!
//! Gradually build PromQL expressions using [Selector], turn it into a [RangeVector] or [InstantVector],
//! apply additional [aggregations] or [functions] on them and evaluate the final expression at an instant ([Client::query])
//! or a range of time ([Client::query_range]).
//!
//! ```rust
//! use prometheus_http_query::{Client, Selector, RangeVector, Aggregate};
//! use prometheus_http_query::aggregations::sum;
//! use prometheus_http_query::functions::rate;
//! use std::convert::TryInto;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), prometheus_http_query::Error> {
//!     let client: Client = Default::default();
//!
//!     let v: RangeVector = Selector::new()
//!         .metric("node_cpu_seconds_total")?
//!         .with("mode", "user")
//!         .range("5m")?
//!         .try_into()?;
//!
//!     let q = sum(rate(v), Some(Aggregate::By(&["cpu"])));
//!
//!     let response = client.query(q, None, None).await?;
//!
//!     assert!(response.as_instant().is_some());
//!    
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
//! use prometheus_http_query::{Client, RangeVector};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), prometheus_http_query::Error> {
//!     let client: Client = Default::default();
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
//! use prometheus_http_query::{Client, Scheme, Selector, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let client = Client::new(Scheme::Http, "localhost", 9090);
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
//!     let response = tokio_test::block_on( async { client.series(&set, None, None).await });
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
//! use prometheus_http_query::{Client, Scheme, Error, RuleType};
//!
//! fn main() -> Result<(), Error> {
//!     let client = Client::new(Scheme::Http, "localhost", 9090);
//!
//!     let response = tokio_test::block_on( async { client.rules(None).await });
//!
//!     assert!(response.is_ok());
//!
//!     // Only request alerting rules instead:
//!     let response = tokio_test::block_on( async { client.rules(Some(RuleType::Alert)).await });
//!
//!     assert!(response.is_ok());
//!
//!     // Request active alerts:
//!     let response = tokio_test::block_on( async { client.alerts().await });
//!
//!     assert!(response.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! This is just one example. See [Client] for examples of other types of metadata queries.
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
//! - [ ] Target metadata (still experimental as of Prometheus v2.28)
//! - [ ] Metric metadata (still experimental as of Prometheus v2.28)
//! - [ ] Alertmanager service discovery status
//! - [ ] Prometheus config
//! - [ ] Prometheus runtime & build information (still unstable as of Prometheus v2.28)
//!
//! # Notes
//!
//! If the JSON response from the Prometheus HTTP API indicates an error (field `status` == `"error"`),
//! then the contents of both fields `errorType` and `error` are captured and then returned by the client
//! as a variant of the [Error] enum, just as any HTTP errors (non-200) that may indicate a problem
//! with the provided query string. Thus any syntax problems etc. that cannot be caught at compile time
//! or before executing the query will at least be propagated in the same manner.
//!
//! # Limitations
//!
//! * Some query types (e.g. Prometheus status, metric & target metadata) are not supported (yet)
//! * reqwest client configuration cannot be customized (yet)
//! * Subqueries are not supported (only as custom query)
//! * PromQL functions that do not take a range / instant vector as an argument are not supported (only as custom query)
pub mod aggregations;
mod client;
mod error;
pub mod functions;
pub mod response;
mod selector;
mod util;
mod vector;
pub use self::client::Client;
pub use self::client::Scheme;
pub use self::error::Error;
pub use self::selector::Selector;
pub use self::util::Aggregate;
pub use self::util::Group;
pub use self::util::Match;
pub use self::util::RuleType;
pub use self::util::TargetState;
pub use self::vector::InstantVector;
pub use self::vector::RangeVector;
