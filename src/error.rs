use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[derive(Debug)]
pub enum Error {
    InvalidTimeDuration,
    IllegalTimeSeriesSelector,
    InvalidRangeVector,
    Reqwest(reqwest::Error),
    ResponseError(ResponseError),
    UnsupportedQueryResultType(UnsupportedQueryResultType),
    UnknownResponseStatus(UnknownResponseStatus),
    InvalidFunctionArgument(InvalidFunctionArgument),
    UrlParse(url::ParseError),
    ResponseParse(serde_json::Error),
    MissingField(MissingFieldError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
            Self::IllegalTimeSeriesSelector => IllegalTimeSeriesSelectorError.fmt(f),
            Self::InvalidRangeVector => InvalidRangeVectorError.fmt(f),
            Self::Reqwest(e) => e.fmt(f),
            Self::ResponseError(e) => e.fmt(f),
            Self::UnsupportedQueryResultType(e) => e.fmt(f),
            Self::UnknownResponseStatus(e) => e.fmt(f),
            Self::InvalidFunctionArgument(e) => e.fmt(f),
            Self::UrlParse(e) => e.fmt(f),
            Self::ResponseParse(e) => e.fmt(f),
            Self::MissingField(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

/// This error is thrown when a time duration is invalidated or empty.<br>
/// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations)
/// for the correct time duration syntax.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidTimeDurationError;

impl fmt::Display for InvalidTimeDurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided time duration is invalid as it does not comply with PromQL time duration syntax")
    }
}

/// This error is thrown when a [Selector] cannot be contructed from the
/// provided metric name and/or the list of labels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalTimeSeriesSelectorError;

// error message was shamelessly copied from the PromQL documentation.
impl fmt::Display for IllegalTimeSeriesSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vector selectors must either specify a name or at least one label matcher that does not match the empty string")
    }
}

/// This error is thrown when a [RangeVector] cannot be contructed from a
/// given [Selector] configuration, e.g. due to a missing time duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidRangeVectorError;

impl fmt::Display for InvalidRangeVectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "range vectors must contain a time duration")
    }
}

/// This error is thrown when the JSON response's `status` field contains `error`.<br>
/// The error-related information in the response is included in this error.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseError {
    pub(crate) kind: String,
    pub(crate) message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "the JSON response contains an error of type {}: {}",
            self.kind, self.message
        )
    }
}

/// This error is thrown when the JSON response's `data.resultType` field contains
/// an unexpected result type.<br>
/// For instant and range queries this is expected to be either `vector` or `matrix`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnsupportedQueryResultType(pub String);

impl fmt::Display for UnsupportedQueryResultType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let UnsupportedQueryResultType(data_type) = self;
        write!(f, "the API returned an unsupported result type, is '{}', must be either 'vector' or 'matrix'", data_type)
    }
}

/// This error is thrown when the JSON response's `status` field contains an
/// unexpected value. As per the Prometheus reference this must be either `success` or `error`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownResponseStatus(pub String);

impl fmt::Display for UnknownResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let UnknownResponseStatus(status) = self;
        write!(f, "the API returned an unknown response status , is '{}', must be either 'success' or 'error'", status)
    }
}

/// This error is thrown whenever arguments supplied to [functions] have
/// invalid values and would result in an API error.
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidFunctionArgument {
    pub(crate) message: String,
}

impl fmt::Display for InvalidFunctionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// This error is thrown when a field is unexpectedly not part of the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct MissingFieldError(pub &'static str);

impl fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MissingFieldError(field) = self;
        write!(
            f,
            "expected field '{}' is missing from the JSON payload",
            field
        )
    }
}
