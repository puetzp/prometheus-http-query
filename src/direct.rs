use crate::client::*;
use crate::error::Error;
use crate::response::*;
use crate::selector::Selector;
use crate::util::{RuleType, TargetState};
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
pub fn query(host: &str, query: impl std::string::ToString) -> Result<InstantQueryBuilder, Error> {
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
    query: impl std::string::ToString,
    start: i64,
    end: i64,
    step: f64,
) -> Result<RangeQueryBuilder, Error> {
    Client::from_str(host).map(|c| c.query_range(query, start, end, step))
}

/// Find time series that match certain label sets ([`Selector`]s).
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
///     let response = series("http://localhost:9090", &[select], None, None).await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn series(
    host: &str,
    selectors: &[Selector<'_>],
    start: Option<i64>,
    end: Option<i64>,
) -> Result<Vec<HashMap<String, String>>, Error> {
    Client::from_str(host)?.series(selectors, start, end).await
}

/// Retrieve all label names (or use [Selector]s to select time series to read label names from).
///
/// This is just a convenience function for one-off requests, see [`Client::label_names`].
///
/// ```rust
/// use prometheus_http_query::label_names;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = label_names("http://localhost:9090", None, None, None).await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn label_names(
    host: &str,
    selectors: Option<Vec<Selector<'_>>>,
    start: Option<i64>,
    end: Option<i64>,
) -> Result<Vec<String>, Error> {
    Client::from_str(host)?
        .label_names(selectors, start, end)
        .await
}

/// Retrieve all label values for a label name (or use [`Selector`]s to select the time series to read label values from)
///
/// This is just a convenience function for one-off requests, see [`Client::label_values`].
///
/// ```rust
/// use prometheus_http_query::label_values;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = label_values("http://localhost:9090", "job", None, None, None).await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn label_values(
    host: &str,
    label: &str,
    selectors: Option<Vec<Selector<'_>>>,
    start: Option<i64>,
    end: Option<i64>,
) -> Result<Vec<String>, Error> {
    Client::from_str(host)?
        .label_values(label, selectors, start, end)
        .await
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

/// Retrieve a list of rule groups of recording and alerting rules.
///
/// This is just a convenience function for one-off requests, see [Client::rules].
///
/// ```rust
/// use prometheus_http_query::{rules, RuleType};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = rules("http://localhost:9090", Some(RuleType::Alert)).await;
///
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn rules(host: &str, rule_type: Option<RuleType>) -> Result<Vec<RuleGroup>, Error> {
    Client::from_str(host)?.rules(rule_type).await
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

/// Retrieve metadata about metrics that are currently scraped from targets, along with target information.
///
/// This is just a convenience function for one-off requests, see [`Client::target_metadata`].
///
/// ```rust
/// use prometheus_http_query::{target_metadata, Selector};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = target_metadata("http://localhost:9090", Some("go_routines"), None, None).await;
///     assert!(response.is_ok());
///
///     let select = Selector::new().eq("job", "prometheus");
///     let response = target_metadata("http://localhost:9090", None, Some(&select), None).await;
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn target_metadata(
    host: &str,
    metric: Option<&str>,
    match_target: Option<&Selector<'_>>,
    limit: Option<usize>,
) -> Result<Vec<TargetMetadata>, Error> {
    Client::from_str(host)?
        .target_metadata(metric, match_target, limit)
        .await
}

/// Retrieve metadata about metrics that are currently scraped from targets.
///
/// This is just a convenience function for one-off requests, see [`Client::metric_metadata`].
///
/// ```rust
/// use prometheus_http_query::{metric_metadata, Selector};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let response = metric_metadata("http://localhost:9090", None, None).await;
///     assert!(response.is_ok());
///
///     let response = metric_metadata("http://localhost:9090", Some("go_routines"), None).await;
///     assert!(response.is_ok());
///
///     Ok(())
/// }
/// ```
pub async fn metric_metadata(
    host: &str,
    metric: Option<&str>,
    limit: Option<usize>,
) -> Result<HashMap<String, Vec<MetricMetadata>>, Error> {
    Client::from_str(host)?.metric_metadata(metric, limit).await
}
