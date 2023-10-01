use crate::selector::Selector;
use crate::Client;
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Wraps errors from the underlying [reqwest::Client] that cannot be mapped
    /// to a more specific error type.
    Client {
        message: &'static str,
        source: Option<reqwest::Error>,
    },
    /// Occurs when Prometheus responds with e.g. HTTP 4xx (e.g. due to a syntax error in a PromQL query).<br>
    /// Details on the error as reported by Prometheus are included in [PrometheusError].
    Prometheus(PrometheusError),
    /// Occurs when the [Client::series] method is called with an empty set of
    /// series [Selector]s. According to the Prometheus API description at least one
    /// [Selector] must be provided.
    EmptySeriesSelector,
    ParseUrl {
        message: &'static str,
        source: url::ParseError,
    },
    ParseResponse {
        message: &'static str,
        source: serde_json::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Client { message, source: _ } => f.write_str(&message),
            Self::Prometheus(e) => e.fmt(f),
            Self::EmptySeriesSelector => f.write_str("at least one series selector must be provided in order to query the series endpoint"),
            Self::ParseUrl {message, source: _ } => f.write_str(&message),
            Self::ParseResponse { message, source: _ } => f.write_str(&message),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Client { message: _, source } => source.as_ref().map(|e| e as &dyn StdError),
            Self::Prometheus(_) => None,
            Self::EmptySeriesSelector => None,
            Self::ParseUrl { message: _, source } => Some(source),
            Self::ParseResponse { message: _, source } => Some(source),
        }
    }
}

/// This error is thrown when the JSON response's `status` field contains `error`.<br>
/// The error-related information from the JSON body is included in this error.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PrometheusError {
    #[serde(alias = "errorType")]
    pub(crate) error_type: PrometheusErrorType,
    #[serde(alias = "error")]
    pub(crate) message: String,
}

impl fmt::Display for PrometheusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl PrometheusError {
    /// Returns the parsed version of the error type that was given by the Prometheus API.
    pub fn error_type(&self) -> PrometheusErrorType {
        self.error_type
    }

    /// Returns the error message that was given by the Prometheus API.
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn is_timeout(&self) -> bool {
        self.error_type == PrometheusErrorType::Timeout
    }

    pub fn is_canceled(&self) -> bool {
        self.error_type == PrometheusErrorType::Canceled
    }

    pub fn is_execution(&self) -> bool {
        self.error_type == PrometheusErrorType::Execution
    }

    pub fn is_bad_data(&self) -> bool {
        self.error_type == PrometheusErrorType::BadData
    }

    pub fn is_internal(&self) -> bool {
        self.error_type == PrometheusErrorType::Internal
    }

    pub fn is_unavailable(&self) -> bool {
        self.error_type == PrometheusErrorType::Unavailable
    }

    pub fn is_not_found(&self) -> bool {
        self.error_type == PrometheusErrorType::NotFound
    }
}

/// The parsed error type as returned by the Prometheus API.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum PrometheusErrorType {
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

impl fmt::Display for PrometheusErrorType {
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
