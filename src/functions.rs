//! A set of PromQL function equivalents e.g. `abs` and `rate`
use crate::error::{Error, InvalidFunctionArgument};
use crate::vector::*;

macro_rules! create_function {
    ( $(#[$attr:meta])* => $func_name:ident, $source_type:ident, $result_type:ident ) => {
        $(#[$attr])*
        pub fn $func_name(vector: $source_type) -> $result_type {
            let $source_type(query) = vector;
            let new = format!("{}({})", stringify!($func_name), query);
            $result_type(new)
        }
    };
}

create_function! {
    /// Apply the PromQL `abs` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::abs;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = abs(vector - 2.0);
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
    => abs, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `absent` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::absent;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "foobar")
    ///         .try_into()?;
    ///
    ///     let q = absent(vector);
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
    => absent, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `absent_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::absent_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "foobar")
    ///         .range("5m")?
    ///         .try_into()?;
    ///
    ///     let q = absent_over_time(vector);
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
    => absent_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `ceil` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::ceil;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = ceil(vector / 2.0);
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
    => ceil, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `changes` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::changes;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = changes(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => changes, RangeVector, InstantVector
}

/// Apply the PromQL `clamp` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::clamp;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = clamp(vector * 5.0, 0.0, 3.0);
///
///     let response = client.query(q, None, None).await?;
///     let value = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .sample()
///         .value();
///
///     assert_eq!(value, 3.0);
///     Ok(())
/// }
/// ```
pub fn clamp(vector: InstantVector, min: f64, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp({}, {}, {})", query, min, max);
    InstantVector(new)
}

/// Apply the PromQL `clamp_max` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::clamp_max;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = clamp_max(vector * 5.0, 3.0);
///
///     let response = client.query(q, None, None).await?;
///     let value = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .sample()
///         .value();
///
///     assert_eq!(value, 3.0);
///     Ok(())
/// }
/// ```
pub fn clamp_max(vector: InstantVector, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp_max({}, {})", query, max);
    InstantVector(new)
}

/// Apply the PromQL `clamp_min` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::clamp_min;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = clamp_min(vector, 5.0);
///
///     let response = client.query(q, None, None).await?;
///     let value = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .sample()
///         .value();
///
///     assert_eq!(value, 5.0);
///     Ok(())
/// }
/// ```
pub fn clamp_min(vector: InstantVector, min: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp_min({}, {})", query, min);
    InstantVector(new)
}

create_function! {
    /// Apply the PromQL `day_of_month` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{day_of_month, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = day_of_month(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((1.0..=31.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => day_of_month, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `day_of_week` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{day_of_week, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = day_of_week(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((0.0..=6.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => day_of_week, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `days_in_month` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{days_in_month, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = days_in_month(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((28.0..=31.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => days_in_month, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `delta` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::delta;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = delta(vector);
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
    => delta, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `deriv` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::deriv;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = deriv(vector);
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
    => deriv, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `exp` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::exp;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = exp(vector);
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
    => exp, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `floor` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::floor;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = floor(vector / 2.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => floor, InstantVector, InstantVector
}

/// Apply the PromQL `histogram_quantile` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, RangeVector};
/// use prometheus_http_query::functions::{histogram_quantile, rate};
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: RangeVector = Selector::new()
///         .metric("prometheus_http_request_duration_seconds_bucket")
///         .with("job", "prometheus")
///         .with("handler", "/metrics")
///         .range("1m")?
///         .try_into()?;
///
///     let q = histogram_quantile(0.9, rate(vector));
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
pub fn histogram_quantile(quantile: f64, vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("histogram_quantile({}, {})", quantile, query);
    InstantVector(new)
}

/// Apply the PromQL `holt_winters` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, RangeVector};
/// use prometheus_http_query::functions::holt_winters;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: RangeVector = Selector::new()
///         .metric("prometheus_http_requests_total")
///         .with("job", "prometheus")
///         .with("handler", "/metrics")
///         .range("1m")?
///         .try_into()?;
///
///     let q = holt_winters(vector, 0.8, 0.9)?;
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
pub fn holt_winters(vector: RangeVector, sf: f64, tf: f64) -> Result<InstantVector, Error> {
    if sf <= 0.0 || tf <= 0.0 || sf >= 1.0 || tf >= 1.0 {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from(
                "smoothing factors in holt_winters() must be between 0.0 (excl.) and 1.0 (excl.)",
            ),
        }));
    }

    let RangeVector(query) = vector;
    let new = format!("holt_winters({}, {}, {})", query, sf, tf);
    Ok(InstantVector(new))
}

create_function! {
    /// Apply the PromQL `hour` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{hour, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = hour(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((0.0..=23.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => hour, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `idelta` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::idelta;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .with("job", "prometheus")
    ///         .with("handler", "/metrics")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = idelta(vector);
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
    => idelta, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `increase` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::increase;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .with("job", "prometheus")
    ///         .with("handler", "/metrics")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = increase(vector);
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
    => increase, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `irate` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::irate;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .with("job", "prometheus")
    ///         .with("handler", "/metrics")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = irate(vector);
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
    => irate, RangeVector, InstantVector
}

/// Apply the PromQL `lable_join` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::label_join;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = label_join(vector, "example_label", " on ", &["job", "instance"])?;
///
///     let response = client.query(q, None, None).await?;
///     let label = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .metric()
///         .get(&"example_label".to_string());
///
///     assert_eq!(label, Some(&"prometheus on localhost:9090".to_string()));
///     Ok(())
/// }
/// ```
pub fn label_join(
    vector: InstantVector,
    dst_label: &str,
    separator: &str,
    src_labels: &[&str],
) -> Result<InstantVector, Error> {
    if dst_label.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("destination label name in label_join() cannot be empty"),
        }));
    }

    if src_labels.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("list of source label names in label_join() cannot be empty"),
        }));
    }

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

    Ok(InstantVector(new))
}

/// Apply the PromQL `lable_replace` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::label_replace;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = label_replace(vector, "example_label", "$1", "instance", "(.*):.*")?;
///
///     let response = client.query(q, None, None).await?;
///     let label = response.as_instant()
///         .unwrap()
///         .get(0)
///         .unwrap()
///         .metric()
///         .get(&"example_label".to_string());
///
///     assert_eq!(label, Some(&"localhost".to_string()));
///     Ok(())
/// }
/// ```
pub fn label_replace(
    vector: InstantVector,
    dst_label: &str,
    replacement: &str,
    src_label: &str,
    regex: &str,
) -> Result<InstantVector, Error> {
    if dst_label.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("destination label name in label_replace() cannot be empty"),
        }));
    }

    let InstantVector(query) = vector;
    let new = format!(
        "label_replace({}, \"{}\", \"{}\", \"{}\", \"{}\")",
        query, dst_label, replacement, src_label, regex
    );
    Ok(InstantVector(new))
}

create_function! {
    /// Apply the PromQL `ln` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::ln;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = ln(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => ln, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `log2` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::log2;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = log2(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => log2, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `log10` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::log10;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = log10(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => log10, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `minute` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{minute, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = minute(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((0.0..=59.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => minute, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `month` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{month, timestamp};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = month(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!((1.0..=12.0).contains(&value));
    ///     Ok(())
    /// }
    /// ```
    => month, InstantVector, InstantVector
}

/// Apply the PromQL `predict_linear` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, RangeVector};
/// use prometheus_http_query::functions::predict_linear;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: RangeVector = Selector::new()
///         .metric("prometheus_http_requests_total")
///         .range("1m")?
///         .try_into()?;
///
///     let q = predict_linear(vector, 300.0);
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
pub fn predict_linear(vector: RangeVector, seconds: f64) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("predict_linear({}, {})", query, seconds);
    InstantVector(new)
}

create_function! {
    /// Apply the PromQL `rate` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::rate;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = rate(vector);
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
    => rate, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `resets` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::resets;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1d")?
    ///         .try_into()?;
    ///
    ///     let q = resets(vector);
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
    => resets, RangeVector, InstantVector
}

/// Apply the PromQL `round` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, InstantVector};
/// use prometheus_http_query::functions::round;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: InstantVector = Selector::new()
///         .metric("up")
///         .with("job", "prometheus")
///         .try_into()?;
///
///     let q = round(vector, None);
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
pub fn round(vector: InstantVector, to_nearest: Option<f64>) -> InstantVector {
    let InstantVector(query) = vector;
    let new = if let Some(nearest) = to_nearest {
        format!("round({}, {})", query, nearest)
    } else {
        format!("round({})", query)
    };
    InstantVector(new)
}

create_function! {
    /// Apply the PromQL `scalar` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::scalar;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = scalar(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_scalar()
    ///         .unwrap()
    ///         .value();
    ///
    ///     assert_eq!(value, 1.0);
    ///     Ok(())
    /// }
    /// ```
    => scalar, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sgn` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::sgn;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = sgn(vector);
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
    => sgn, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sort` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::sort;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = sort(vector);
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
    => sort, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sort_desc` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::sort_desc;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = sort_desc(vector);
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
    => sort_desc, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `timestamp` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::timestamp;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = timestamp(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!(value > 1644417828.0);
    ///     Ok(())
    /// }
    /// ```
    => timestamp, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `year` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{timestamp, year};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .try_into()?;
    ///
    ///     let q = year(timestamp(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!(value >= 2022.0);
    ///     Ok(())
    /// }
    /// ```
    => year, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `avg_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::avg_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = avg_over_time(vector);
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
    => avg_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `min_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::min_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = min_over_time(vector);
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
    => min_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `max_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::max_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = max_over_time(vector);
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
    => max_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sum_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::sum_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = sum_over_time(vector);
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
    => sum_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `count_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::count_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = count_over_time(vector);
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
    => count_over_time, RangeVector, InstantVector
}

/// Apply the PromQL `quantile_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Client, Selector, RangeVector};
/// use prometheus_http_query::functions::quantile_over_time;
/// use std::convert::TryInto;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), prometheus_http_query::Error> {
///     let client = Client::default();
///     let vector: RangeVector = Selector::new()
///         .metric("prometheus_http_requests_total")
///         .range("1m")?
///         .try_into()?;
///
///     let q = quantile_over_time(0.95, vector);
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
pub fn quantile_over_time(quantile: f64, vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("quantile_over_time({}, {})", quantile, query);
    InstantVector(new)
}

create_function! {
    /// Apply the PromQL `stddev_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::stddev_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = stddev_over_time(vector);
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
    => stddev_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `stdvar_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::stdvar_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = stdvar_over_time(vector);
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
    => stdvar_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `last_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::last_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = last_over_time(vector);
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
    => last_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `present_over_time` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, RangeVector};
    /// use prometheus_http_query::functions::present_over_time;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client = Client::default();
    ///     let vector: RangeVector = Selector::new()
    ///         .metric("prometheus_http_requests_total")
    ///         .range("1m")?
    ///         .try_into()?;
    ///
    ///     let q = present_over_time(vector);
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
    => present_over_time, RangeVector, InstantVector
}

create_function! {
    /// Apply the PromQL `acos` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::acos;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = acos(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => acos, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `acosh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::acosh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = acosh(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => acosh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `asin` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::asin;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = asin(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => asin, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `asinh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::asinh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = asinh(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => asinh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `atan` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::atan;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = atan(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => atan, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `atanh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::atanh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = atanh(vector);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!(value.is_infinite());
    ///     Ok(())
    /// }
    /// ```
    => atanh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `cos` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::cos;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = cos(vector - 1.0);
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
    => cos, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `cosh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::cosh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = cosh(vector - 1.0);
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
    => cosh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sin` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::sin;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = sin(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => sin, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `sinh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::sinh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = sinh(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => sinh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `tan` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::tan;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = tan(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => tan, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `tanh` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::tanh;
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = tanh(vector - 1.0);
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 0.0);
    ///     Ok(())
    /// }
    /// ```
    => tanh, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `deg` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{atan,deg};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = deg(atan(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert_eq!(value, 45.0);
    ///     Ok(())
    /// }
    /// ```
    => deg, InstantVector, InstantVector
}

create_function! {
    /// Apply the PromQL `rad` function.
    ///
    /// ```rust
    /// use prometheus_http_query::{Client, Selector, InstantVector};
    /// use prometheus_http_query::functions::{atanh,rad};
    /// use std::convert::TryInto;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let client: Client = Default::default();
    ///     let vector: InstantVector = Selector::new()
    ///         .metric("up")
    ///         .with("job", "prometheus")
    ///         .try_into()?;
    ///
    ///     let q = rad(atanh(vector));
    ///
    ///     let response = client.query(q, None, None).await?;
    ///     let value = response.as_instant()
    ///         .unwrap()
    ///         .get(0)
    ///         .unwrap()
    ///         .sample()
    ///         .value();
    ///
    ///     assert!(value.is_infinite());
    ///     Ok(())
    /// }
    /// ```
    => rad, InstantVector, InstantVector
}
