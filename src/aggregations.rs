//! A set of aggregation operators like `sum` and `avg`
use crate::util::*;
use crate::vector::*;

/// Use the `sum` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::sum;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("http_requests_total")?
///         .with("job", "apiserver")
///         .try_into()?;
///
///     let result = sum(vector, Some(Aggregate::By(&["code"])));
///
///     assert_eq!(result.to_string(), String::from("sum by (code) (http_requests_total{job=\"apiserver\"})"));
///
///     Ok(())
/// }
/// ```
pub fn sum(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("sum {} ({})", l.to_string(), old_vec),
        None => format!("sum ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `min` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::min;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = min(vector, Some(Aggregate::By(&["cpu"])));
///
///     assert_eq!(result.to_string(), String::from("min by (cpu) (node_cpu_seconds_total)"));
///
///     Ok(())
/// }
/// ```
pub fn min(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("min {} ({})", l.to_string(), old_vec),
        None => format!("min ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `max` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::max;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = max(vector, Some(Aggregate::By(&["cpu"])));
///
///     assert_eq!(result.to_string(), String::from("max by (cpu) (node_cpu_seconds_total)"));
///
///     Ok(())
/// }
/// ```
pub fn max(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("max {} ({})", l.to_string(), old_vec),
        None => format!("max ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `avg` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::aggregations::avg;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_memory_Active_bytes")?
///         .try_into()?;
///
///     let result = avg(vector, None);
///
///     assert_eq!(result.to_string(), String::from("avg (node_memory_Active_bytes)"));
///
///     Ok(())
/// }
/// ```
pub fn avg(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("avg {} ({})", l.to_string(), old_vec),
        None => format!("avg ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `group` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::group;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = group(vector, Some(Aggregate::Without(&["mode"])));
///
///     assert_eq!(result.to_string(), String::from("group without (mode) (node_cpu_seconds_total)"));
///
///     Ok(())
/// }
/// ```
pub fn group(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("group {} ({})", l.to_string(), old_vec),
        None => format!("group ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `stddev` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::stddev;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("promhttp_metric_handler_requests_total")?
///         .try_into()?;
///
///     let result = stddev(vector, Some(Aggregate::By(&["code"])));
///
///     assert_eq!(result.to_string(), String::from("stddev by (code) (promhttp_metric_handler_requests_total)"));
///
///     Ok(())
/// }
/// ```
pub fn stddev(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("stddev {} ({})", l.to_string(), old_vec),
        None => format!("stddev ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `stdvar` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::stdvar;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("promhttp_metric_handler_requests_total")?
///         .try_into()?;
///
///     let result = stdvar(vector, Some(Aggregate::By(&["code"])));
///
///     assert_eq!(result.to_string(), String::from("stdvar by (code) (promhttp_metric_handler_requests_total)"));
///
///     Ok(())
/// }
/// ```
pub fn stdvar(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("stdvar {} ({})", l.to_string(), old_vec),
        None => format!("stdvar ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `count` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::aggregations::count;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = count(vector, None);
///
///     assert_eq!(result.to_string(), String::from("count (node_cpu_seconds_total)"));
///
///     Ok(())
/// }
/// ```
pub fn count(vector: InstantVector, labels: Option<Aggregate<'_>>) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("count {} ({})", l.to_string(), old_vec),
        None => format!("count ({})", old_vec),
    };

    InstantVector(new_vec)
}

/// Use the `count_values` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::aggregations::count_values;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("promhttp_metric_handler_requests_total")?
///         .try_into()?;
///
///     let result = count_values(vector, None, "http_code");
///
///     assert_eq!(result.to_string(), String::from("count_values (\"http_code\", promhttp_metric_handler_requests_total)"));
///
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
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::aggregations::bottomk;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = bottomk(vector, None, 2);
///
///     assert_eq!(result.to_string(), String::from("bottomk (2, node_cpu_seconds_total)"));
///
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
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::aggregations::topk;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = topk(vector, None, 2);
///
///     assert_eq!(result.to_string(), String::from("topk (2, node_cpu_seconds_total)"));
///
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
/// use prometheus_http_query::{Selector, InstantVector, Aggregate};
/// use prometheus_http_query::aggregations::quantile;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("node_cpu_seconds_total")?
///         .try_into()?;
///
///     let result = quantile(vector, Some(Aggregate::By(&["cpu", "mode"])), 0.1);
///
///     assert_eq!(result.to_string(), String::from("quantile by (cpu,mode) (0.1, node_cpu_seconds_total)"));
///
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
