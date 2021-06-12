use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalMetricName,
    InvalidTimestamp,
    InvalidTimeDuration,
    IllegalTimeSeriesSelector,
    EmptyRange,
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IllegalMetricName => IllegalMetricNameError.fmt(f),
            Self::InvalidTimestamp => InvalidTimestampError.fmt(f),
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
            Self::IllegalTimeSeriesSelector => IllegalTimeSeriesSelectorError.fmt(f),
            Self::EmptyRange => EmptyRangeError.fmt(f),
            Self::Reqwest(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalMetricNameError;

impl fmt::Display for IllegalMetricNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided metric name is a reserved PromQL keyword")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidTimestampError;

impl fmt::Display for InvalidTimestampError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a time parameter to the Prometheus API must be either a UNIX timestamp in seconds (with optional decimal places) or a RFC3339-compatible string")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidTimeDurationError;

impl fmt::Display for InvalidTimeDurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided time duration is invalid as it does not comply with PromQL time duration syntax")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalTimeSeriesSelectorError;

// error message was shamelessly copied from the PromQL documentation.
impl fmt::Display for IllegalTimeSeriesSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vector selectors must either specify a name or at least one label matcher that does not match the empty string")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyRangeError;

impl fmt::Display for EmptyRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided duration must contain a value")
    }
}
