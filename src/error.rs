use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Wraps errors from the underlying [reqwest::Client]
    Client(reqwest::Error),
    /// Occurs when the request was successful but the JSON response contains errors returned by the API
    ApiError(ApiError),
    EmptySeriesSelector,
    UrlParse(url::ParseError),
    ResponseParse(serde_json::Error),
    MissingField(MissingFieldError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Client(e) => e.fmt(f),
            Self::ApiError(e) => e.fmt(f),
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
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ApiError {
    #[serde(alias = "errorType")]
    pub(crate) kind: String,
    #[serde(alias = "error")]
    pub(crate) message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "the API returned error of type {} as part of JSON response: {}",
            self.kind, self.message
        )
    }
}

/// This error is thrown when a field is unexpectedly not part of the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct MissingFieldError(pub(crate) &'static str);

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
