use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Wraps errors from the underlying [reqwest::Client] when e.g. a HTTP 5xx
    /// is returned by Prometheus.
    Client(reqwest::Error),
    /// Occurs when Prometheus responds with HTTP 4xx (e.g. due to a syntax error in a PromQL query).<br>
    /// The details of this error are included in [ApiError].
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
/// The error-related information from the JSON body is included in this error.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ApiError {
    #[serde(alias = "errorType")]
    pub(crate) error_type: ApiErrorType,
    #[serde(alias = "error")]
    pub(crate) message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "the API returned an error of type {}: {}",
            self.error_type, self.message
        )
    }
}

impl ApiError {
    /// Returns the parsed version of the error type as reported by the Prometheus API.
    pub fn error_type(&self) -> ApiErrorType {
        self.error_type
    }

    pub fn is_timeout(&self) -> bool {
        self.error_type == ApiErrorType::Timeout
    }

    pub fn is_canceled(&self) -> bool {
        self.error_type == ApiErrorType::Canceled
    }

    pub fn is_execution(&self) -> bool {
        self.error_type == ApiErrorType::Execution
    }

    pub fn is_bad_data(&self) -> bool {
        self.error_type == ApiErrorType::BadData
    }

    pub fn is_internal(&self) -> bool {
        self.error_type == ApiErrorType::Internal
    }

    pub fn is_unavailable(&self) -> bool {
        self.error_type == ApiErrorType::Unavailable
    }

    pub fn is_not_found(&self) -> bool {
        self.error_type == ApiErrorType::NotFound
    }
}

/// The parsed error type as returned by the Prometheus API.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum ApiErrorType {
    #[serde(alias = "timeout")]
    Timeout,
    #[serde(alias = "canceled")]
    Canceled,
    #[serde(alias = "execution")]
    Execution,
    #[serde(alias = "bad_data")]
    BadData,
    #[serde(alias = "internal")]
    Internal,
    #[serde(alias = "unavailable")]
    Unavailable,
    #[serde(alias = "not_found")]
    NotFound,
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Timeout => write!(f, "timeout"),
            Self::Canceled => write!(f, "canceled"),
            Self::Execution => write!(f, "execution"),
            Self::BadData => write!(f, "bad_data"),
            Self::Internal => write!(f, "internal"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::NotFound => write!(f, "not_found"),
        }
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
