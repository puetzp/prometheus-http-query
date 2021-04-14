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
//!         query: "up",
//!         time: None,
//!         timeout: None,
//!     };
//!
//!     let response = instant_q.execute(&client).await.unwrap();
//!
//!     assert!(response.is_success());
//!
//!     let range_q = RangeQuery {
//!         query: "up",
//!         start: "2021-04-09T11:30:00.000+02:00",
//!         end: "2021-04-09T12:30:00.000+02:00",
//!         step: "5m",
//!         timeout: None,
//!     };
//!
//!     let response = range_q.execute(&client).await.unwrap();
//!
//!     assert!(response.is_success());
//! }
//! ```
pub mod client;
pub mod query;
pub mod response;
pub use self::client::Client;
pub use self::client::Scheme;
pub use self::query::InstantQuery;
pub use self::query::Query;
pub use self::query::RangeQuery;
