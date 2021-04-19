use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuilderError {
    InvalidMetricName,
    InvalidTimeSpecifier,
    InvalidTimeDuration,
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMetricName => InvalidMetricNameError.fmt(f),
            Self::InvalidTimeSpecifier => InvalidTimeSpecifierError.fmt(f),
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
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
