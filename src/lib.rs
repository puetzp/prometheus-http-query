//! This crate provides an interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/).
//! The [`Client`] is used to interact with a Prometheus server. It is basically a wrapper around a [`reqwest::Client`] and implements
//! additional methods to execute PromQL queries and fetch metadata.
//!
//! # Usage
//!
//! The following code contains just a few examples. See [`Client`] for the complete set of available functions.
//!
//!
//! ## Initialize a client
//!
//! The [`Client`] can be constructed in various ways depending on your need to add customizations.
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
//! use prometheus_http_query::Client;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), anyhow::Error> {
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
//! Retrieve a list of time series that match a certain label set by providing one or more time series [`Selector`]s
//! or providing selectors in their native string syntax, depending on what better suits your use case.<br>
//! More metadata queries are available, e.g. methods to retrieve label names and values.
//!
//! ```rust
//! use prometheus_http_query::{Client, Selector};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let client = Client::default();
//!
//!     // Using `Selector`.
//!     {
//!         let s1 = Selector::new()
//!             .eq("handler", "/api/v1/query");
//!
//!         let s2 = Selector::new()
//!             .eq("job", "node")
//!             .regex_eq("mode", ".+");
//!
//!         let response = client.series().match_many(&[s1, s2]).get().await;
//!
//!         assert!(response.is_ok());
//!     }
//!
//!     // Using the free-form API.
//!     {
//!         let response = client.series()
//!             .match_many(&["{handler=\"/api/v1/query\"}", "{job=\"node\", mode=\".+\"}"])
//!             .get()
//!             .await;
//!
//!         assert!(response.is_ok());
//!     }
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
//! use prometheus_http_query::{Client, RuleKind};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let client = Client::default();
//!
//!     let response = client.rules().get().await;
//!
//!     assert!(response.is_ok());
//!
//!     // Only request alerting rules instead:
//!     let response = client.rules().kind(RuleKind::Alerting).get().await;
//!
//!     assert!(response.is_ok());
//!
//!     // Request active alerts:
//!     let response = client.alerts().get().await;
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
//! use prometheus_http_query::{query, runtime_information};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let q = "topk by (code) (5, prometheus_http_requests_total)";
//!     let response = query("http://localhost:9090", q)?.get().await?;
//!
//!     assert!(response.data().as_vector().is_some());
//!
//!     let response = runtime_information("http://localhost:9090")?.get().await;
//!
//!     assert!(response.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! At this point all available feature flags pertain to the [`Client`]s TLS configuration. They enable feature flags of
//! the `reqwest` crate by the same name.<br>
//! See the [reqwest documentation](https://docs.rs/reqwest/0.11.14/reqwest/index.html#optional-features) for details on
//! these feature flags.<br>
//! Also make sure that default features of `prometheus-http-query` are disabled if you choose a TLS library other than
//! the default:
//!
//! `prometheus-http-query = { version = "0.7", default-features = false, features = ["rustls-tls"] }`
//!
//! # Compatibility
//!
//! The crate is generally compatible with Prometheus server >=2.30. However individual [`Client`] methods might
//! only work with the latest Prometheus server version when the corresponding API endpoint has only recently
//! been introduced.<br>
//! The minimum recommended Prometheus server version is v2.46.<br>
//! Also some features may only work when the Prometheus server is started with certain flags. An example
//! are query statistics that can be enabled via [`RangeQueryBuilder::stats`]. The response
//! will not contain per-step stats unless Prometheus is started with `--enable-feature=promql-per-step-stats`.
//!
//! # Error handling
//!
//! All [`Client`] methods that interact with the Prometheus API return a `Result`. Also each request to the API
//! may fail at different stages. In general the following approach is taken to return the most significant
//! error to the caller:
//! - When the server's response contains header `Content-Type: application/json` (or variants thereof) the
//! JSON body is parsed to the target type, regardless of the HTTP status code, since Prometheus returns elaborate
//! error messages within the HTTP body in any case.
//! A JSON response having `"status": "success"` is deserialized to the target type of this function and returned
//! within `Result::Ok`. A response with `"status": "error"` is instead deserialized to a [`error::PrometheusError`]
//! and returned within `Result::Err`.
//! - Any other server HTTP 4xx/5xx responses without the proper header indicating a JSON-encoded body are
//! returned as [`Error::Client`] within `Result::Err`. For example, this may happen when an intermediate proxy server
//! fails to handle a request and subsequently return a plain text error message and a non-2xx HTTP status code.
//!
//! # Supported operations
//!
//! - [x] [Instant queries](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries) via [`Client::query`]
//! - [x] [Range queries](https://prometheus.io/docs/prometheus/latest/querying/api/#range-queries) via [`Client::query_range`]
//! - [ ] [Query cost estimation](https://prometheus.io/docs/prometheus/latest/querying/api/#query-cost)
//! - [ ] [Formatting query expressions](https://prometheus.io/docs/prometheus/latest/querying/api/#formatting-query-expressions)
//! - [ ] [Parsing PromQL expression into an abstract syntax tree](https://prometheus.io/docs/prometheus/latest/querying/api/#parsing-a-promql-expressions-into-a-abstract-syntax-tree-ast)
//! - [x] [Finding series by label matchers](https://prometheus.io/docs/prometheus/latest/querying/api/#finding-series-by-label-matchers) via [`Client::series`]
//! - [x] [Getting label names](https://prometheus.io/docs/prometheus/latest/querying/api/#getting-label-names) via [`Client::label_names`]
//! - [x] [Querying label values](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-label-values) via [`Client::label_values`]
//! - [ ] [Searching metric names, label names and label values](https://prometheus.io/docs/prometheus/latest/querying/api/#searching-metric-names-label-names-and-label-values)
//! - [ ] [Querying exemplars](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-exemplars)
//! - [ ] [Scrape pools](https://prometheus.io/docs/prometheus/latest/querying/api/#scrape-pools)
//! - [x] [Targets](https://prometheus.io/docs/prometheus/latest/querying/api/#targets) via [`Client::targets`]
//! - [ ] [Relabel steps](https://prometheus.io/docs/prometheus/latest/querying/api/#relabel-steps)
//! - [x] [Rules](https://prometheus.io/docs/prometheus/latest/querying/api/#rules) via [`Client::rules`]
//! - [x] [Alerts](https://prometheus.io/docs/prometheus/latest/querying/api/#alerts) via [`Client::alerts`]
//! - [x] [Querying target metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-target-metadata) via [`Client::target_metadata`]
//! - [x] [Querying metric metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-metric-metadata) via [`Client::metric_metadata`]
//! - [x] [Alertmanagers](https://prometheus.io/docs/prometheus/latest/querying/api/#alertmanagers) via [`Client::alertmanagers`]
//! - [ ] [Prometheus server config](https://prometheus.io/docs/prometheus/latest/querying/api/#config)
//! - [x] [Prometheus server flags](https://prometheus.io/docs/prometheus/latest/querying/api/#flags) via [`Client::flags`]
//! - [x] [Prometheus server runtime information](https://prometheus.io/docs/prometheus/latest/querying/api/#runtime-information) via [`Client::runtime_information`]
//! - [x] [Prometheus server build information](https://prometheus.io/docs/prometheus/latest/querying/api/#build-information) via [`Client::build_information`]
//! - [x] [Prometheus server TSDB stats](https://prometheus.io/docs/prometheus/latest/querying/api/#tsdb-stats) via [`Client::tsdb_statistics`]
//! - [ ] [Prometheus server TSDB blocks](https://prometheus.io/docs/prometheus/latest/querying/api/#tsdb-blocks)
//! - [x] [WAL Replay Stats](https://prometheus.io/docs/prometheus/latest/querying/api/#wal-replay-stats) via [`Client::wal_replay_statistics`]
//! - [ ] [Prometheus server self metrics](https://prometheus.io/docs/prometheus/latest/querying/api/#self-metrics)
//! - [x] [Prometheus server health](https://prometheus.io/docs/prometheus/latest/management_api/#health-check) via [`Client::is_server_healthy`]
//! - [x] [Prometheus server readiness](https://prometheus.io/docs/prometheus/latest/management_api/#readiness-check) via [`Client::is_server_ready`]
//!
//! # Limitations
//!
//! * Some [`Client`] methods may not work with older versions of the Prometheus server.
//! * The [String](https://prometheus.io/docs/prometheus/latest/querying/api/#strings) result type is not supported
//! as it is currently not used by Prometheus.
//! * Warnings contained in an API response will be ignored.
mod client;
mod direct;
pub mod error;
pub mod response;
mod selector;
mod util;
pub use self::client::{
    AlertmanagersQueryBuilder, AlertsQueryBuilder, BuildInformationQueryBuilder, Client,
    FlagsQueryBuilder, InstantQueryBuilder, LabelNamesQueryBuilder, LabelValuesQueryBuilder,
    MetricMetadataQueryBuilder, RangeQueryBuilder, RulesQueryBuilder,
    RuntimeInformationQueryBuilder, SeriesQueryBuilder, TargetMetadataQueryBuilder,
    TargetsQueryBuilder,
};
pub use self::direct::*;
pub use self::error::Error;
pub use self::selector::Selector;
pub use self::util::AlertState;
pub use self::util::Duration;
pub use self::util::RuleHealth;
pub use self::util::RuleKind;
pub use self::util::TargetHealth;
pub use self::util::TargetState;

// Re-exported for users' convenience.
pub use reqwest;
