//! A set of aggregation operators like `sum` and `avg`
use crate::util::*;
use crate::vector::*;

macro_rules! create_aggregation {
    ( $(#[$attr:meta])* => $func_name:ident ) => {
        $(#[$attr])*
        pub fn $func_name(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
            let InstantVector(old_vec) = vector;

            let new_vec = match labels {
                Some(l) => format!("{} {} ({})", stringify!($func_name), l.to_string(), old_vec),
                None => format!("{} ({})", stringify!($func_name), old_vec),
            };

            InstantVector(new_vec)
        }
    };
}

create_aggregation! {
    /// Use the `sum` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector, Aggregate};
    /// use prometheus_http_query::aggregations::sum;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = sum(vector, Some(Aggregate::By(&["code"])));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => sum
}

create_aggregation! {
    /// Use the `min` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector, Aggregate};
    /// use prometheus_http_query::aggregations::min;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = min(vector, Some(Aggregate::By(&["code"])));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => min
}

create_aggregation! {
    /// Use the `max` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector, Aggregate};
    /// use prometheus_http_query::aggregations::max;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = max(vector, Some(Aggregate::By(&["code"])));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => max
}

create_aggregation! {
    /// Use the `avg` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::aggregations::avg;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = avg(vector, None);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => avg
}

create_aggregation! {
    /// Use the `group` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::aggregations::group;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = group(vector, None);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 1.0);
    ///     Ok(())
    /// }
    /// ```
    => group
}

create_aggregation! {
    /// Use the `stddev` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::aggregations::stddev;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = stddev(vector, None);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => stddev
}

create_aggregation! {
    /// Use the `stdvar` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::aggregations::stdvar;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = stdvar(vector, None);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let first_item = response.as_instant()
    ///         .unwrap()
    ///         .get(0);
    ///
    ///     assert!(first_item.is_some());
    ///     Ok(())
    /// }
    /// ```
    => stdvar
}

create_aggregation! {
    /// Use the `count` aggregation operator on an instant vector.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::aggregations::count;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = count(vector, None);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!(value.is_normal());
    ///     Ok(())
    /// }
    /// ```
    => count
}

/// Use the `count_values` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::aggregations::count_values;
/// use prometheus_http_query::functions::round;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("prometheus_target_interval_length_seconds")
///         .try_into()?;
///
///     let q = count_values(round(vector, None), None, "interval_length");
///
///     let response = client.query(q, None, None).await?;
///     let value = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .sample()
///         .value();
///
///     assert!(value.is_normal());
///     Ok(())
/// }
/// ```
pub fn count_values<'a>(
    vector: InstantVector,
    labels: Option<Aggregate<'a>>,
    parameter: &'a str,
) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!(
            "count_values {} (\"{}\", {})",
            l.to_string(),
            parameter,
            old_vec
        ),
        None => format!("count_values (\"{}\", {})", parameter, old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `bottomk` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::aggregations::bottomk;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("prometheus_engine_query_duration_seconds")
///         .try_into()?;
///
///     let q = bottomk(vector, None, 5);
///
///     let response = client.query(q, None, None).await?;
///
///     assert!(response.as_instant().is_some());
///     Ok(())
/// }
/// ```
pub fn bottomk(
    vector: InstantVector,
    labels: Option<Aggregate<'_>>,
    parameter: u64,
) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("bottomk {} ({}, {})", l.to_string(), parameter, old_vec),
        None => format!("bottomk ({}, {})", parameter, old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `topk` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::aggregations::topk;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("prometheus_engine_query_duration_seconds")
///         .try_into()?;
///
///     let q = topk(vector, None, 5);
///
///     let response = client.query(q, None, None).await?;
///
///     assert!(response.as_instant().is_some());
///     Ok(())
/// }
/// ```
pub fn topk(vector: InstantVector, labels: Option<Aggregate<'_>>, parameter: u64) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("topk {} ({}, {})", l.to_string(), parameter, old_vec),
        None => format!("topk ({}, {})", parameter, old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `quantile` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::quantile;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("prometheus_target_interval_length_seconds")
///         .try_into()?;
///
///     let q = quantile(vector, Some(Aggregate::By(&["prepare_time"])), 0.9);
///
///     let response = client.query(q, None, None).await?;
///     let value = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .sample()
///         .value();
///
///     assert!(value.is_normal());
///     Ok(())
/// }
/// ```
pub fn quantile(
    vector: InstantVector,
    labels: Option<Aggregate<'_>>,
    parameter: f64,
) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("quantile {} ({}, {})", l.to_string(), parameter, old_vec),
        None => format!("quantile ({}, {})", parameter, old_vec),
    };

    InstantVector(new_vec)
}
