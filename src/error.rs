use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    ResponseError(ResponseError),
    UnsupportedQueryResultType(UnsupportedQueryResultType),
    UnknownResponseStatus(UnknownResponseStatus),
    EmptySeriesSelector,
    UrlParse(url::ParseError),
    ResponseParse(serde_json::Error),
    MissingField(MissingFieldError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Reqwest(e) => e.fmt(f),
            Self::ResponseError(e) => e.fmt(f),
            Self::UnsupportedQueryResultType(e) => e.fmt(f),
            Self::UnknownResponseStatus(e) => e.fmt(f),
            Self::EmptySeriesSelector => write!(f, "at least one series selector must be provided in order to query the series endpoint"),
            Self::UrlParse(e) => e.fmt(f),
            Self::ResponseParse(e) => e.fmt(f),
            Self::MissingField(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

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
