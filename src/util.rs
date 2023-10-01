use crate::error::Error;
use mime::Mime;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use std::fmt;
use url::Url;

/// A helper type to filter targets by state.
#[derive(Debug)]
pub enum TargetState {
    Active,
    Dropped,
    Any,
}

impl fmt::Display for TargetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetState::Active => write!(f, "active"),
            TargetState::Dropped => write!(f, "dropped"),
            TargetState::Any => write!(f, "any"),
        }
    }
}

/// A helper type to represent possible target health states.
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum TargetHealth {
    #[serde(alias = "up")]
    Up,
    #[serde(alias = "down")]
    Down,
    #[serde(alias = "unknown")]
    Unknown,
}

impl fmt::Display for TargetHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetHealth::Up => write!(f, "up"),
            TargetHealth::Down => write!(f, "down"),
            TargetHealth::Unknown => write!(f, "unknown"),
        }
    }
}

/// A helper type to represent possible rule health states.
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum RuleHealth {
    #[serde(alias = "ok")]
    Good,
    #[serde(alias = "err")]
    Bad,
    #[serde(alias = "unknown")]
    Unknown,
}

impl fmt::Display for RuleHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleHealth::Good => write!(f, "ok"),
            RuleHealth::Bad => write!(f, "err"),
            RuleHealth::Unknown => write!(f, "unknown"),
        }
    }
}

/// A helper type to represent possible rule health states.
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum AlertState {
    #[serde(alias = "inactive")]
    Inactive,
    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "firing")]
    Firing,
}

impl fmt::Display for AlertState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertState::Inactive => write!(f, "inactive"),
            AlertState::Pending => write!(f, "pending"),
            AlertState::Firing => write!(f, "firing"),
        }
    }
}

/// A helper type to filter rules by type.
#[derive(Debug)]
pub enum RuleType {
    Alert,
    Record,
}

impl fmt::Display for RuleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleType::Alert => write!(f, "alert"),
            RuleType::Record => write!(f, "record"),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Label<'a> {
    Equal((&'a str, &'a str)),
    NotEqual((&'a str, &'a str)),
    RegexEqual((&'a str, &'a str)),
    RegexNotEqual((&'a str, &'a str)),
}

impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Label::Equal((k, v)) => write!(f, "{}=\"{}\"", k, v),
            Label::NotEqual((k, v)) => write!(f, "{}!=\"{}\"", k, v),
            Label::RegexEqual((k, v)) => write!(f, "{}=~\"{}\"", k, v),
            Label::RegexNotEqual((k, v)) => write!(f, "{}!~\"{}\"", k, v),
        }
    }
}

/// Create a base URL that is common to all queries from a string literal.
/// The implementations are probably a little more complicated than they need
/// to be as we need to allocate a new string during the URL construction
/// to preserve a possibly existing path component in the source string.
pub(crate) trait ToBaseUrl {
    fn to_base_url(self) -> Result<Url, Error>;
}

impl ToBaseUrl for &str {
    fn to_base_url(self) -> Result<Url, Error> {
        Url::parse(self).map_err(|source| Error::ParseUrl {
            message: "failed to build Prometheus server base URL",
            source,
        })
    }
}

impl ToBaseUrl for String {
    fn to_base_url(self) -> Result<Url, Error> {
        Url::parse(&self).map_err(|source| Error::ParseUrl {
            message: "failed to build Prometheus server base URL",
            source,
        })
    }
}

pub(crate) fn build_final_url(mut url: Url, path: &str) -> Url {
    let base_path = url.path();
    match base_path {
        "/" => return url.join(path).unwrap(),
        _ => {
            let p = format!("{}/{}", base_path, path);
            url.set_path(&p);
        }
    }
    url
}

pub(crate) fn is_json(v: Option<&HeaderValue>) -> bool {
    match v
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.parse::<Mime>().ok())
    {
        Some(mime) => match (mime.type_(), mime.subtype()) {
            (mime::APPLICATION, mime::JSON) => true,
            _ => false,
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{build_final_url, is_json, ToBaseUrl};

    #[test]
    fn test_simple_str_to_url() {
        let s = "http://127.0.0.1:9090";
        let url = s.to_base_url().unwrap();
        assert_eq!("http://127.0.0.1:9090/", url.as_str());
    }

    #[test]
    fn test_proxied_str_to_url() {
        let s = "http://proxy.example.com/prometheus";
        let url = s.to_base_url().unwrap();
        assert_eq!("http://proxy.example.com/prometheus", url.as_str());
    }

    #[test]
    fn test_simple_string_to_url() {
        let s = String::from("http://127.0.0.1:9090");
        let url = s.to_base_url().unwrap();
        assert_eq!("http://127.0.0.1:9090/", url.as_str());
    }

    #[test]
    fn test_proxied_string_to_url() {
        let s = String::from("http://proxy.example.com/prometheus");
        let url = s.to_base_url().unwrap();
        assert_eq!("http://proxy.example.com/prometheus", url.as_str());
    }

    #[test]
    fn test_simple_url_finalization() {
        let s = String::from("http://127.0.0.1:9090");
        let url = s.to_base_url().unwrap();
        let final_url = build_final_url(url, "api/v1/targets");
        assert_eq!("http://127.0.0.1:9090/api/v1/targets", final_url.as_str());
    }

    #[test]
    fn test_proxied_url_finalization() {
        let s = String::from("http://proxy.example.com/prometheus");
        let url = s.to_base_url().unwrap();
        let final_url = build_final_url(url, "api/v1/targets");
        assert_eq!(
            "http://proxy.example.com/prometheus/api/v1/targets",
            final_url.as_str()
        );
    }

    #[test]
    fn test_is_json() {
        let header = reqwest::header::HeaderValue::from_static("application/json");
        assert!(is_json(Some(&header)));
    }

    #[test]
    fn test_is_json_with_charset() {
        let header = reqwest::header::HeaderValue::from_static("application/json; charset=utf-8");
        assert!(is_json(Some(&header)));
    }
}
