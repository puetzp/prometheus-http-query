use crate::vector::*;

/// Apply the PromQL `abs` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::abs;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = abs(vector);
///
///     assert_eq!(&result.to_string(), "abs(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn abs(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("abs({})", query);
    InstantVector(new)
}

/// Apply the PromQL `absent` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::absent;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("nonexistent")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = absent(vector);
///
///     assert_eq!(&result.to_string(), "absent(nonexistent{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn absent(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("absent({})", query);
    InstantVector(new)
}

/// Apply the PromQL `absent_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::absent_over_time;
/// use prometheus_http_query::RangeVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("nonexistent")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = absent_over_time(vector);
///
///     assert_eq!(&result.to_string(), "absent_over_time(nonexistent{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn absent_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("absent_over_time({})", query);
    RangeVector(new)
}

/// Apply the PromQL `ceil` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::ceil;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = ceil(vector);
///
///     assert_eq!(&result.to_string(), "ceil(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn ceil(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("ceil({})", query);
    InstantVector(new)
}

/// Apply the PromQL `changes` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::changes;
/// use prometheus_http_query::RangeVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = changes(vector);
///
///     assert_eq!(&result.to_string(), "changes(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn changes(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("changes({})", query);
    RangeVector(new)
}

/// Apply the PromQL `clamp` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, Aggregate};
/// use prometheus_http_query::functions::clamp;
/// use prometheus_http_query::InstantVector;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = clamp(vector, 0.5, 0.75);
///
///     assert_eq!(&result.to_string(), "clamp(some_metric{some_label=\"some_value\"}, 0.5, 0.75)");
///
///     Ok(())
/// }
/// ```
pub fn clamp(vector: InstantVector, min: f64, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp({}, {}, {})", query, min, max);
    InstantVector(new)
}

pub fn clamp_max(vector: InstantVector, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp({}, {})", query, max);
    InstantVector(new)
}

pub fn clamp_min(vector: InstantVector, min: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp({}, {})", query, min);
    InstantVector(new)
}

pub fn day_of_month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("day_of_month({})", query);
    InstantVector(new)
}

pub fn day_of_week(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("day_of_week({})", query);
    InstantVector(new)
}

pub fn days_in_month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("days_in_month({})", query);
    InstantVector(new)
}

pub fn delta(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("delta({})", query);
    RangeVector(new)
}

pub fn deriv(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("deriv({})", query);
    RangeVector(new)
}

pub fn exp(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("exp({})", query);
    InstantVector(new)
}

pub fn floor(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("floor({})", query);
    InstantVector(new)
}

pub fn histogram_quantile(quantile: f64, vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("floor({}, {})", quantile, query);
    InstantVector(new)
}

pub fn holt_winters(vector: RangeVector, sf: f64, tf: f64) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("holt_winters({}, {}, {})", query, sf, tf);
    RangeVector(new)
}

pub fn hour(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("hour({})", query);
    InstantVector(new)
}

pub fn idelta(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("idelta({})", query);
    RangeVector(new)
}

pub fn increase(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("increase({})", query);
    RangeVector(new)
}

pub fn irate(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("irate({})", query);
    RangeVector(new)
}

pub fn label_join(
    vector: InstantVector,
    dst_label: &str,
    separator: &str,
    src_labels: &[&str],
) -> InstantVector {
    let InstantVector(query) = vector;

    let src_labels = src_labels
        .iter()
        .map(|l| format!("\"{}\"", l))
        .collect::<Vec<String>>()
        .join(", ");

    let new = format!(
        "label_join({}, \"{}\", \"{}\", {})",
        query, dst_label, separator, src_labels
    );

    InstantVector(new)
}

pub fn label_replace(
    vector: InstantVector,
    dst_label: &str,
    replacement: &str,
    src_label: &str,
    regex: &str,
) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!(
        "label_replace({}, \"{}\", \"{}\", \"{}\", \"{}\")",
        query, dst_label, replacement, src_label, regex
    );
    InstantVector(new)
}

pub fn ln(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("ln({})", query);
    InstantVector(new)
}

pub fn log2(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("log2({})", query);
    InstantVector(new)
}

pub fn log10(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("log10({})", query);
    InstantVector(new)
}

pub fn minute(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("minute({})", query);
    InstantVector(new)
}

pub fn month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("month({})", query);
    InstantVector(new)
}

pub fn predict_linear(vector: RangeVector, seconds: f64) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("predict_linear({}, {})", query, seconds);
    RangeVector(new)
}

pub fn rate(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("rate({})", query);
    RangeVector(new)
}

pub fn resets(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("resets({})", query);
    RangeVector(new)
}

pub fn round(vector: InstantVector, to_nearest: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("round({}, {})", query, to_nearest);
    InstantVector(new)
}

pub fn scalar(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("scalar({})", query);
    InstantVector(new)
}

pub fn sgn(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sgn({})", query);
    InstantVector(new)
}

pub fn sort(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sort({})", query);
    InstantVector(new)
}

pub fn sort_desc(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sort_desc({})", query);
    InstantVector(new)
}

pub fn timestamp(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("timestamp({})", query);
    InstantVector(new)
}

pub fn year(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("year({})", query);
    InstantVector(new)
}

pub fn avg_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("avg_over_time({})", query);
    RangeVector(new)
}

pub fn min_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("min_over_time({})", query);
    RangeVector(new)
}

pub fn max_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("max_over_time({})", query);
    RangeVector(new)
}

pub fn sum_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("sum_over_time({})", query);
    RangeVector(new)
}

pub fn count_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("count_over_time({})", query);
    RangeVector(new)
}

pub fn quantile_over_time(quantile: f64, vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("quantile_over_time({}, {})", quantile, query);
    RangeVector(new)
}

pub fn stddev_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("stddev_over_time({})", query);
    RangeVector(new)
}

pub fn stdvar_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("stdvar_over_time({})", query);
    RangeVector(new)
}

pub fn last_over_time(vector: RangeVector) -> RangeVector {
    let RangeVector(query) = vector;
    let new = format!("last_over_time({})", query);
    RangeVector(new)
}
