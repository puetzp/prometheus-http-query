//! This crate provides a query interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/).
//! At this point only instant and range queries are supported. But this will definitely change in the future
//! and include all the remaining types of queries that the API specifies.
//!
//! Behind the scenes this library uses `reqwest` as a HTTP client. Thus its features and limitations also
//! apply to this library.
//!
//! # Usage
//! ```rust
//! use prometheus_http_query::{Client, Query, RangeQuery, InstantQuery};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let client: Client = Default::default();
//!
//!     let instant_q = InstantQuery {
//!         query: "up".to_string(),
//!         time: None,
//!         timeout: None,
//!     };
//!
//!     let response = instant_q.execute(&client).await.unwrap();
//!
//!     assert!(response.is_success());
//!
//!     let range_q = RangeQuery {
//!         query: "up".to_string(),
//!         start: "2021-04-09T11:30:00.000+02:00".to_string(),
//!         end: "2021-04-09T12:30:00.000+02:00".to_string(),
//!         step: "5m".to_string(),
//!         timeout: None,
//!     };
//!
//!     let response = range_q.execute(&client).await.unwrap();
//!
//!     assert!(response.is_success());
//! }
//! ```
//!
//! # Notes
//!
//! The response types `InstantQueryResponse` and `RangeQueryResponse` do not exactly match
//! the deserialzed version of the JSON that Prometheus sends back to the client. The reasoning
//! behind this is that there is only one possible type of response to e.g. an instant query, namely
//! a vector of metrics. So the structure of the response is dumbed down in order to get rid of
//! redundant information that the JSON contains, like the resultType.
//!
//! # Future plans
//!
//! * Add metadata queries (series/labels) (non-breaking)
//! * Expose configuration parameters of the reqwest client
//! * Provide types for the values of e.g. InstantQuery.query or RangeQuery.start in order to
//! catch possible errors with the format before actually sending the request to a remote Prometheus
//! instance (breaking)
//! * The best way of preventing issues with the query format would be to provide a builder type to
//! build a query and translate it to a query string to be sent to Prometheus (breaking)
pub mod builder;
pub mod client;
pub mod error;
pub mod operators;
pub mod query;
pub mod response;
pub mod selector;
mod util;
pub use self::builder::QueryBuilder;
pub use self::client::Client;
pub use self::client::Scheme;
pub use self::error::BuilderError;
pub use self::query::InstantQuery;
pub use self::query::Query;
pub use self::query::RangeQuery;
pub use self::selector::Selector;
pub use self::util::LabelList;
