use crate::selector::Selector;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InstantVector(pub(crate) String);

impl fmt::Display for InstantVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let InstantVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for InstantVector {
    type Error = crate::error::Error;

    /// Convert a `Selector` to an `InstantVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    fn try_from(selector: Selector) -> Result<InstantVector, Self::Error> {
        if selector.labels.is_none() && selector.metric.is_none() {
            return Err(crate::error::Error::IllegalTimeSeriesSelector);
        }

        let selector_str = selector.to_string();
        Ok(InstantVector(selector_str))
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeVector(pub(crate) String);

impl fmt::Display for RangeVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let RangeVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for RangeVector {
    type Error = crate::error::Error;

    /// Convert a `Selector` to an `RangeVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::RangeVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<RangeVector, Error> = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .range("1m30s")
    ///     .unwrap()
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    fn try_from(selector: Selector) -> Result<RangeVector, Self::Error> {
        if selector.labels.is_none() && selector.metric.is_none() {
            return Err(crate::error::Error::IllegalTimeSeriesSelector);
        }

        if selector.duration.is_none() {
            return Err(crate::error::Error::InvalidRangeVector);
        };

        Ok(RangeVector(selector.to_string()))
    }
}
