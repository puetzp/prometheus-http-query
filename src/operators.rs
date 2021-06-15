use crate::util::*;
use crate::vector::*;

/// Use the `sum` aggregation operator on an instant vector.
///
/// ```rust
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::sum;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("http_requests_total")
///     .unwrap()
///     .with("job", "apiserver")
///     .try_into()
///     .unwrap();
///
/// let result = sum(vector, Some(LabelList::By(&["code"])));
///
/// assert_eq!(result.to_string(), String::from("sum by (code) (http_requests_total{job=\"apiserver\"})"))
/// ```
pub fn sum<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::min;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = min(vector, Some(LabelList::By(&["cpu"])));
///
/// assert_eq!(result.to_string(), String::from("min by (cpu) (node_cpu_seconds_total)"))
/// ```
pub fn min<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::max;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = max(vector, Some(LabelList::By(&["cpu"])));
///
/// assert_eq!(result.to_string(), String::from("max by (cpu) (node_cpu_seconds_total)"))
/// ```
pub fn max<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::avg;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_memory_Active_bytes")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = avg(vector, None);
///
/// assert_eq!(result.to_string(), String::from("avg (node_memory_Active_bytes)"))
/// ```
pub fn avg<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::group;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = group(vector, Some(LabelList::Without(&["mode"])));
///
/// assert_eq!(result.to_string(), String::from("group without (mode) (node_cpu_seconds_total)"))
/// ```
pub fn group<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::stddev;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("promhttp_metric_handler_requests_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = stddev(vector, Some(LabelList::By(&["code"])));
///
/// assert_eq!(result.to_string(), String::from("stddev by (code) (promhttp_metric_handler_requests_total)"))
/// ```
pub fn stddev<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::stdvar;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("promhttp_metric_handler_requests_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = stdvar(vector, Some(LabelList::By(&["code"])));
///
/// assert_eq!(result.to_string(), String::from("stdvar by (code) (promhttp_metric_handler_requests_total)"))
/// ```
pub fn stdvar<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::count;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = count(vector, None);
///
/// assert_eq!(result.to_string(), String::from("count (node_cpu_seconds_total)"))
/// ```
pub fn count<'a>(vector: InstantVector, labels: Option<LabelList<'a>>) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::count_values;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("promhttp_metric_handler_requests_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = count_values(vector, None, "http_code");
///
/// assert_eq!(result.to_string(), String::from("count_values (\"http_code\", promhttp_metric_handler_requests_total)"))
/// ```
pub fn count_values<'a>(
    vector: InstantVector,
    labels: Option<LabelList<'a>>,
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::bottomk;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = bottomk(vector, None, 2);
///
/// assert_eq!(result.to_string(), String::from("bottomk (2, node_cpu_seconds_total)"))
/// ```
pub fn bottomk<'a>(
    vector: InstantVector,
    labels: Option<LabelList<'a>>,
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::topk;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = topk(vector, None, 2);
///
/// assert_eq!(result.to_string(), String::from("topk (2, node_cpu_seconds_total)"))
/// ```
pub fn topk<'a>(
    vector: InstantVector,
    labels: Option<LabelList<'a>>,
    parameter: u64,
) -> InstantVector {
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
/// use prometheus_http_query::{Selector, LabelList};
/// use prometheus_http_query::operators::quantile;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// let vector: InstantVector = Selector::new()
///     .metric("node_cpu_seconds_total")
///     .unwrap()
///     .try_into()
///     .unwrap();
///
/// let result = quantile(vector, Some(LabelList::By(&["cpu", "mode"])), 0.1);
///
/// assert_eq!(result.to_string(), String::from("quantile by (cpu,mode) (0.1, node_cpu_seconds_total)"))
/// ```
pub fn quantile<'a>(
    vector: InstantVector,
    labels: Option<LabelList<'a>>,
    parameter: f64,
) -> InstantVector {
    let InstantVector(old_vec) = vector;

    let new_vec = match labels {
        Some(l) => format!("quantile {} ({}, {})", l.to_string(), parameter, old_vec),
        None => format!("quantile ({}, {})", parameter, old_vec),
    };

    InstantVector(new_vec)
}
