//! The goal of this crate is to provide a query interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/) and leverage Rust's type system in the process. Thus mistakes while building a query can be caught at compile-time (or at least before actually sending the query to Prometheus).
//!
//! Most of the features of PromQL are mirrored in this library. Queries are gradually built from time series selectors, aggregations
//! and functions and then passed to an HTTP client to execute.
//!
//! Behind the scenes this library uses the [reqwest] crate as a HTTP client. Thus its features and limitations also
//! apply to this library.
//!
//! # Usage
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
//!     let q = sum(rate(v), Some(Aggregate::By(&["cpu"]))) ;
//!
//!     let response = client.query(q, None, None).await;
//!
//!     assert!(response.is_ok());
//!
//!     // It is also possible to bypass every kind of validation by supplying
//!     // a custom query directly to the InstantVector | RangeVector types.
//!     // The equivalent of the operation above would be:
//!     let q = r#"sum by(cpu) (rate(node_cpu_seconds_total{mode="user"}[5m]))"#;
//!
//!     let v = RangeVector(q.to_string());
//!
//!     let response = client.query(v, None, None).await;
//!
//!     assert!(response.is_ok());
//!    
//!     Ok(())
//! }
//! ```
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
//! * Metadata queries (series/labels) are not supported (yet)
//! * reqwest client configuration cannot be customized (yet)
//! * Subqueries are not supported (only as custom query)
//! * PromQL functions that do not take a range / instant vector as an argument are not supported (only as custom query)
pub mod aggregations;
mod client;
mod error;
pub mod functions;
mod response;
mod selector;
mod util;
mod vector;
pub use self::client::Client;
pub use self::client::Scheme;
pub use self::error::Error;
pub use self::response::*;
pub use self::selector::Selector;
pub use self::util::Aggregate;
pub use self::util::Group;
pub use self::util::Match;
pub use self::vector::InstantVector;
pub use self::vector::RangeVector;
