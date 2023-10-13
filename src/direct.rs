use crate::client::*;
use crate::error::Error;
use crate::response::*;
use crate::selector::Selector;
use crate::util::TargetState;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::FromStr;

/// Execute an instant query.
///
/// This is just a convenience function for one-off requests, see [`Client::query`].
///
/// ```rust
/// use prometheus_http_query::query;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let q = "sum(prometheus_http_requests_total)";
///
///     let response = query("http://localhost:9090", q)?.timeout(1000).get().await?;
///
///     assert!(response.data().as_vector().is_some());
///
///     // Or make a POST request.
///     let response = query("http://localhost:9090", q)?.timeout(1000).post().await?;
///
///     assert!(response.data().as_vector().is_some());
///
///     Ok(())
/// }
/// ```
pub fn query(host: &str, query: impl std::fmt::Display) -> Result<InstantQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.query(query))
}

/// Execute a range query.
///
/// This is just a convenience function for one-off requests, see [`Client::query_range`].
///
/// ```rust
/// use prometheus_http_query::query_range;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let q = "sum(prometheus_http_requests_total)";
///
///     let response = query_range("http://localhost:9090", q, 1648373100, 1648373300, 10.0)?.get().await?;
///
///     assert!(response.data().as_matrix().is_some());
///
///     // Or make a POST request.
///     let response = query_range("http://localhost:9090", q, 1648373100, 1648373300, 10.0)?.post().await?;
///
///     assert!(response.data().as_matrix().is_some());
///
///     Ok(())
/// }
/// ```
pub fn query_range(
    host: &str,
    query: impl std::fmt::Display,
    start: i64,
    end: i64,
    step: f64,
) -> Result<RangeQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.query_range(query, start, end, step))
}

/// Create a [`SeriesQueryBuilder`] to apply filters to a series metadata
/// query before sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [`Client::series`].
///
/// ```rust
/// use prometheus_http_query::{series, Selector};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let select = Selector::new()
///         .eq("handler", "/api/v1/query");
///
///     let response = series("http://localhost:9090", &[select])?.get().await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn series<'a, T>(host: &str, selectors: T) -> Result<SeriesQueryBuilder, Error>
where
    T: IntoIterator,
    T::Item: Borrow<Selector<'a>>,
{
    Client::from_str(host).and_then(|c| c.series(selectors))
}

/// Create a [`LabelNamesQueryBuilder`] to apply filters to a query for the label
/// names endpoint before sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [`Client::label_names`].
///
/// ```rust
/// use prometheus_http_query::label_names;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = label_names("http://localhost:9090")?.get().await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn label_names(host: &str) -> Result<LabelNamesQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.label_names())
}

/// Create a [`LabelValuesQueryBuilder`] to apply filters to a query for the label
/// values endpoint before sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [`Client::label_values`].
///
/// ```rust
/// use prometheus_http_query::label_values;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = label_values("http://localhost:9090", "job")?.get().await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn label_values(host: &str, label: &str) -> Result<LabelValuesQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.label_values(label))
}

/// Query the current state of target discovery.
///
/// This is just a convenience function for one-off requests, see [`Client::targets`].
///
/// ```rust
/// use prometheus_http_query::{targets, TargetState};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = targets("http://localhost:9090", Some(TargetState::Active)).await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn targets(host: &str, state: Option<TargetState>) -> Result<Targets, Error> {
    Client::from_str(host)?.targets(state).await
}

/// Create a [`RulesQueryBuilder`] to apply filters to the rules query before
/// sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [Client::rules].
///
/// ```rust
/// use prometheus_http_query::{rules, RuleKind};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = rules("http://localhost:9090")?.kind(RuleKind::Recording).get().await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn rules(host: &str) -> Result<RulesQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.rules())
}

/// Retrieve a list of active alerts.
///
/// This is just a convenience function for one-off requests, see [`Client::alerts`].
///
/// ```rust
/// use prometheus_http_query::alerts;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = alerts("http://localhost:9090").await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn alerts(host: &str) -> Result<Vec<Alert>, Error> {
    Client::from_str(host)?.alerts().await
}

/// Retrieve a list of flags that Prometheus was configured with.
///
/// This is just a convenience function for one-off requests, see [`Client::flags`].
///
/// ```rust
/// use prometheus_http_query::flags;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = flags("http://localhost:9090").await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn flags(host: &str) -> Result<HashMap<String, String>, Error> {
    Client::from_str(host)?.flags().await
}

/// Retrieve Prometheus server build information.
///
/// This is just a convenience function for one-off requests, see [`Client::build_information`].
///
/// ```rust
/// use prometheus_http_query::build_information;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = build_information("http://localhost:9090").await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn build_information(host: &str) -> Result<BuildInformation, Error> {
    Client::from_str(host)?.build_information().await
}

/// Retrieve Prometheus server runtime information.
///
/// This is just a convenience function for one-off requests, see [`Client::runtime_information`].
///
/// ```rust
/// use prometheus_http_query::runtime_information;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = runtime_information("http://localhost:9090").await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn runtime_information(host: &str) -> Result<RuntimeInformation, Error> {
    Client::from_str(host)?.runtime_information().await
}

/// Query the current state of alertmanager discovery.
///
/// This is just a convenience function for one-off requests, see [`Client::alertmanagers`].
///
/// ```rust
/// use prometheus_http_query::alertmanagers;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = alertmanagers("http://localhost:9090").await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn alertmanagers(host: &str) -> Result<Alertmanagers, Error> {
    Client::from_str(host)?.alertmanagers().await
}

/// Create a [`TargetMetadataQueryBuilder`] to apply filters to a target metadata
/// query before sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [`Client::target_metadata`].
///
/// ```rust
/// use prometheus_http_query::{target_metadata, Selector};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = target_metadata("http://localhost:9090")?
///         .metric("go_goroutines")
///         .get()
///         .await;
///     assert!(response.is_ok());
///
///     let select = Selector::new().eq("job", "prometheus");
///     let response = target_metadata("http://localhost:9090")?
///         .match_target(&select)
///         .get()
///         .await;
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn target_metadata(host: &str) -> Result<TargetMetadataQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.target_metadata())
}

/// Create a [`MetricMetadataQueryBuilder`] to apply filters to a metric metadata
/// query before sending it to Prometheus.
///
/// This is just a convenience function for one-off requests, see [`Client::metric_metadata`].
///
/// ```rust
/// use prometheus_http_query::{metric_metadata, Selector};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = metric_metadata("http://localhost:9090")?.get().await;
///     assert!(response.is_ok());
///
///     let response = metric_metadata("http://localhost:9090")?
///         .metric("go_goroutines")
///         .get()
///         .await;
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub fn metric_metadata(host: &str) -> Result<MetricMetadataQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.metric_metadata())
}
