use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuilderError {
    InvalidMetricName,
    InvalidTimeSpecifier,
    InvalidTimeDuration,
    InvalidQuery,
    IllegalVectorSelector,
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMetricName => InvalidMetricNameError.fmt(f),
            Self::InvalidTimeSpecifier => InvalidTimeSpecifierError.fmt(f),
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
            Self::InvalidQuery => InvalidQueryError.fmt(f),
            Self::IllegalVectorSelector => IllegalVectorSelectorError.fmt(f),
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
        write!(f, "...TODO...")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidQueryError;

impl fmt::Display for InvalidQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...TODO...")
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
