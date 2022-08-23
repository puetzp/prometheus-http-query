//! This crate provides an interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/) and leverage Rust's type system in the process where applicable.
//!
//! The [Client] uses as [reqwest::Client] as HTTP client internally as you will see in the usage section. Thus its features and limitations also apply to this library.
//!
//! # Usage
//!
//! The following code contains just a few examples. See [Client] for the complete set of query functions.
//!
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
//! ## Execute PromQL queries
//!
//! ```rust
//! use prometheus_http_query::{Client, Error, Selector};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     // Execute a query using HTTP GET.
//!     let q = "topk by (code) (5, prometheus_http_requests_total)";
//!     let response = client.query(q).get().await?;
//!     assert!(response.data().as_vector().is_some());
//!
//!     let q = r#"sum(prometheus_http_requests_total{code="200"})"#;
//!     let response = client.query(q).get().await?;
//!     let result = response.data().as_vector().expect("Expected result of type vector");
//!
//!     if !result.is_empty() {
//!         let first = result.first().unwrap();
//!         println!("Received a total of {} HTTP requests", first.sample().value());
//!     }
//!
//!     // HTTP POST is also supported.
//!     let q = "topk by (code) (5, prometheus_http_requests_total)";
//!     let response = client.query(q).post().await?;
//!     let result = response.data().as_vector().is_some();
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
//!         .eq("handler", "/api/v1/query");
//!
//!     let s2 = Selector::new()
//!         .eq("job", "node")
//!         .regex_eq("mode", ".+");
//!
//!     let response = client.series(&[s1, s2], None, None).await;
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
//! ## Convenience functions for one-off requests
//!
//! ```rust
//! use prometheus_http_query::{Error, query, runtime_information};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let q = "topk by (code) (5, prometheus_http_requests_total)";
//!     let response = query("http://localhost:9090", q)?.get().await?;
//!
//!     assert!(response.data().as_vector().is_some());
//!
//!     let response = runtime_information("http://localhost:9090").await;
//!
//!     assert!(response.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! # Compatibility
//!
//! See the `README` for details on this matter.
//!
//! # Supported operations
//!
//! - [x] Execute instant and range queries (GET or POST) and properly parse the results (vector/matrix/scalar)
//! - [x] Execute series metadata queries
//! - [x] Execute label metadata queries (names/values)
//! - [x] Retrieve target discovery status
//! - [x] Retrieve alerting + recording rules
//! - [x] Retrieve active alerts
//! - [x] Retrieve configured flags & values
//! - [x] Query target metadata
//! - [x] Query metric metadata
//! - [x] Query alertmanager service discovery status
//! - [x] Prometheus server flags
//! - [x] Prometheus server build information
//! - [x] Prometheus server runtime information
//! - [ ] Prometheus server config
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
//! # Limitations
//!
//! * Some [Client] methods may not work with older versions of the Prometheus server
//! * The [String](https://prometheus.io/docs/prometheus/latest/querying/api/#strings) result type is not supported
//! * Warnings contained in a API response will be ignored
mod client;
mod direct;
pub mod error;
pub mod response;
mod selector;
mod util;
pub use self::client::{Client, InstantQueryBuilder, RangeQueryBuilder};
pub use self::direct::*;
pub use self::error::Error;
pub use self::selector::Selector;
pub use self::util::RuleType;
pub use self::util::TargetState;
