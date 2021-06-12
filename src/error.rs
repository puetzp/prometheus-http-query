use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuilderError {
    InvalidMetricName,
    IllegalMetricName,
    InvalidTimeSpecifier,
    InvalidTimeDuration,
    IllegalVectorSelector,
    IllegalRangeVectorSelector,
    EmptyRange,
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMetricName => InvalidMetricNameError.fmt(f),
            Self::IllegalMetricName => IllegalMetricNameError.fmt(f),
            Self::InvalidTimeSpecifier => InvalidTimeSpecifierError.fmt(f),
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
            Self::IllegalVectorSelector => IllegalVectorSelectorError.fmt(f),
            Self::IllegalRangeVectorSelector => IllegalRangeVectorSelectorError.fmt(f),
            Self::EmptyRange => EmptyRangeError.fmt(f),
        }
    }
}

impl Error for BuilderError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidMetricNameError;

impl fmt::Display for InvalidMetricNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided metric name is a reserved PromQL keyword")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalMetricNameError;

impl fmt::Display for IllegalMetricNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided metric name is a reserved PromQL keyword")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidTimeSpecifierError;

impl fmt::Display for InvalidTimeSpecifierError {
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
pub struct IllegalVectorSelectorError;

// error message was shamelessly copied from the PromQL documentation.
impl fmt::Display for IllegalVectorSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vector selectors must either specify a name or at least one label matcher that does not match the empty string")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalRangeVectorSelectorError;

// error message was shamelessly copied from the PromQL documentation.
impl fmt::Display for IllegalRangeVectorSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a range query must have start, end and step parameters")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyRangeError;

impl fmt::Display for EmptyRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided duration must contain a value")
    }
}
