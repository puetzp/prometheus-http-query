use crate::error::{Error, ParseUrlError};
use mime::Mime;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use std::{fmt, str::FromStr};
use time::Duration as TimeDuration;
use url::Url;

/// A wrapper around [`time::Duration`] that is used to deserialize
/// responses or verify time durations before sending them to Prometheus.
///
/// ```rust
/// use prometheus_http_query::{Duration, Client};
/// use std::str::FromStr;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), anyhow::Error> {
///     let client = Client::default();
///
///     let q = "prometheus_http_requests_total";
///     let d = Duration::from_str("1m")?;
///
///     let response = client.query_range(q, 1648373100, 1648373300, d.into()).get().await?;
///
///     assert!(response.data().as_matrix().is_some());
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Duration(TimeDuration);

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut total_milliseconds: i64 = 0;

        // Add each number character to a string until a unit character is encountered.
        // This string is then cleared to process the next number + unit.
        let mut raw_num = String::new();

        // Iterate the duration string, convert each unit to nanoseconds and add
        // it to the total.
        let mut duration_iter = s.chars().peekable();

        // Check for minus sign. It can be safely ignored at this point and the first
        // number parsed as a positive integer because any of the following
        // multiplications will switch the sign automatically.
        let is_neg = duration_iter.next_if_eq(&'-').is_some();

        while let Some(item) = duration_iter.next() {
            if ('0'..='9').contains(&item) {
                raw_num.push(item);
                continue;
            }

            let num = raw_num.parse::<i64>().map_err(Error::ParseInt)?;

            let factor = match item {
                'y' => 1000 * 60 * 60 * 24 * 365,
                'w' => 1000 * 60 * 60 * 24 * 7,
                'd' => 1000 * 60 * 60 * 24,
                'h' => 1000 * 60 * 60,
                'm' => {
                    if duration_iter.next_if_eq(&'s').is_some() {
                        1
                    } else {
                        1000 * 60
                    }
                }
                's' => 1000,
                _ => return Err(Error::Parse(format!("invalid time duration: {}", s))),
            };

            if is_neg {
                total_milliseconds -= num * factor;
            } else {
                total_milliseconds += num * factor;
            }

            raw_num.clear();
        }

        Ok(Self(TimeDuration::milliseconds(total_milliseconds)))
    }
}

impl std::ops::Deref for Duration {
    type Target = TimeDuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Duration {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

impl From<f64> for Duration {
    fn from(f: f64) -> Self {
        Self(TimeDuration::saturating_seconds_f64(f))
    }
}

impl Into<f64> for Duration {
    fn into(self) -> f64 {
        self.as_seconds_f64()
    }
}

impl From<i64> for Duration {
    fn from(s: i64) -> Self {
        Self(TimeDuration::seconds(s))
    }
}

impl Into<i64> for Duration {
    fn into(self) -> i64 {
        self.whole_seconds()
    }
}

/// A helper enum to filter targets by state.
#[derive(Clone, Copy, Debug)]
pub enum TargetState {
    Active,
    Dropped,
    Any,
}

impl fmt::Display for TargetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => f.write_str("active"),
            Self::Dropped => f.write_str("dropped"),
            Self::Any => f.write_str("any"),
        }
    }
}

impl FromStr for TargetState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "dropped" => Ok(Self::Dropped),
            "any" => Ok(Self::Any),
            _ => Err(Error::Parse(format!(
                "failed to parse target state from '{}', must be one of 'active', 'dropped', 'any'",
                s
            ))),
        }
    }
}

/// A helper enum to represent possible target health states.
#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq)]
pub enum TargetHealth {
    #[serde(alias = "up")]
    Up,
    #[serde(alias = "down")]
    Down,
    #[serde(alias = "unknown")]
    Unknown,
}

impl TargetHealth {
    pub fn is_up(&self) -> bool {
        *self == Self::Up
    }

    pub fn is_down(&self) -> bool {
        *self == Self::Down
    }

    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }
}

impl fmt::Display for TargetHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

/// A helper enum to represent possible rule health states.
#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq)]
pub enum RuleHealth {
    #[serde(alias = "ok")]
    Good,
    #[serde(alias = "err")]
    Bad,
    #[serde(alias = "unknown")]
    Unknown,
}

impl RuleHealth {
    pub fn is_good(&self) -> bool {
        *self == Self::Good
    }

    pub fn is_bad(&self) -> bool {
        *self == Self::Bad
    }

    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }
}

impl fmt::Display for RuleHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Good => f.write_str("ok"),
            Self::Bad => f.write_str("err"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

/// A helper type to represent possible rule health states.
#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq)]
pub enum AlertState {
    #[serde(alias = "inactive")]
    Inactive,
    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "firing")]
    Firing,
}

impl AlertState {
    pub fn is_inactive(&self) -> bool {
        *self == Self::Inactive
    }

    pub fn is_pending(&self) -> bool {
        *self == Self::Pending
    }

    pub fn is_firing(&self) -> bool {
        *self == Self::Firing
    }
}

impl fmt::Display for AlertState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => f.write_str("inactive"),
            Self::Pending => f.write_str("pending"),
            Self::Firing => f.write_str("firing"),
        }
    }
}

/// A helper enum to filter rules by type.
#[derive(Copy, Clone, Debug)]
pub enum RuleKind {
    Alerting,
    Recording,
}

impl RuleKind {
    pub(crate) fn to_query_param(&self) -> String {
        match self {
            Self::Alerting => String::from("alert"),
            Self::Recording => String::from("record"),
        }
    }
}

impl fmt::Display for RuleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alerting => f.write_str("alerting"),
            Self::Recording => f.write_str("recording"),
        }
    }
}

impl FromStr for RuleKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alert"| "alerting" => Ok(Self::Alerting),
            "record"| "recording" => Ok(Self::Recording),
            _ => Err(Error::Parse(format!("failed to parse rule type from '{}', must be one of 'alert', 'alerting', 'record', 'recording'", s)))
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
            Self::Equal((k, v)) => write!(f, "{}=\"{}\"", k, v),
            Self::NotEqual((k, v)) => write!(f, "{}!=\"{}\"", k, v),
            Self::RegexEqual((k, v)) => write!(f, "{}=~\"{}\"", k, v),
            Self::RegexNotEqual((k, v)) => write!(f, "{}!~\"{}\"", k, v),
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
        Url::parse(self).map_err(|source| {
            Error::ParseUrl(ParseUrlError {
                message: "failed to build Prometheus server base URL",
                source,
            })
        })
    }
}

impl ToBaseUrl for String {
    fn to_base_url(self) -> Result<Url, Error> {
        Url::parse(&self).map_err(|source| {
            Error::ParseUrl(ParseUrlError {
                message: "failed to build Prometheus server base URL",
                source,
            })
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
    use super::{build_final_url, is_json, Duration, ToBaseUrl};
    use std::str::FromStr;
    use time::Duration as TimeDuration;

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

    #[test]
    fn test_duration() -> Result<(), anyhow::Error> {
        {
            let s = "1s";
            let expected = Duration(TimeDuration::seconds(1));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 1.0);
            assert_eq!(Into::<i64>::into(expected), 1);
        }

        {
            let s = "-1s";
            let expected = Duration(TimeDuration::seconds(-1));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), -1.0);
            assert_eq!(Into::<i64>::into(expected), -1);
        }

        {
            let s = "2m";
            let expected = Duration(TimeDuration::seconds(120));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 120.0);
            assert_eq!(Into::<i64>::into(expected), 120);
        }

        {
            let s = "-2m";
            let expected = Duration(TimeDuration::seconds(-120));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), -120.0);
            assert_eq!(Into::<i64>::into(expected), -120);
        }

        {
            let s = "1ms";
            let expected = Duration(TimeDuration::milliseconds(1));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 0.001);
            assert_eq!(Into::<i64>::into(expected), 0);
        }

        {
            let s = "-1ms";
            let expected = Duration(TimeDuration::milliseconds(-1));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), -0.001);
            assert_eq!(Into::<i64>::into(expected), 0);
        }

        {
            let s = "-2h";
            let expected = Duration(TimeDuration::seconds(-7200));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), -7200.0);
            assert_eq!(Into::<i64>::into(expected), -7200);
        }

        {
            let s = "1h30m";
            let expected = Duration(TimeDuration::seconds(5400));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 5400.0);
            assert_eq!(Into::<i64>::into(expected), 5400);
        }

        {
            let s = "12h34m56s";
            let expected = Duration(TimeDuration::seconds(45296));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 45296.0);
            assert_eq!(Into::<i64>::into(expected), 45296);
        }

        {
            let s = "54s321ms";
            let expected = Duration(TimeDuration::saturating_seconds_f64(54.321));
            assert_eq!(Duration::from_str(s)?, expected);
            assert_eq!(Into::<f64>::into(expected), 54.321);
            assert_eq!(Into::<i64>::into(expected), 54);
        }

        Ok(())
    }
}
