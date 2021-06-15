use crate::error::Error;
use crate::util::*;
use std::fmt;

/// A time series selector that is built from a metric name and/or
/// a set of label matchers using various methods documented below.
///
/// For final validation and further processing the selector is then
/// converted to either a `InstantVector` or `RangeVector`.
///
#[derive(Debug, PartialEq)]
pub struct Selector<'a> {
    pub(crate) metric: Option<&'a str>,
    pub(crate) labels: Option<Vec<Label<'a>>>,
    pub(crate) duration: Option<&'a str>,
}

impl<'a> Selector<'a> {
    /// Simply return an empty `Selector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new().try_into();
    ///
    /// assert!(v.is_err());
    /// ```
    pub fn new() -> Self {
        Selector {
            metric: None,
            labels: None,
            duration: None,
        }
    }

    /// Select a metric name for this `Selector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("http_requests_total");
    ///
    /// assert!(s.is_ok());
    /// ```
    ///
    /// ... which must not be any reserved PromQL keyword.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("group_left");
    ///
    /// assert!(s.is_err());
    /// ```
    pub fn metric(mut self, metric: &'a str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match metric {
            "bool" | "on" | "ignoring" | "group_left" | "group_right" => {
                return Err(Error::IllegalMetricName)
            }
            _ => self.metric = Some(metric),
        }

        Ok(self)
    }

    /// Append a label matcher to the set of matchers of `Selector` that
    /// selects labels that match the provided string.<br>
    /// PromQL example: `http_requests_total{job="apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("http_requests_total")
    ///     .unwrap()
    ///     .with("job", "apiserver")
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    pub fn with(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::With((label, value))),
            None => self.labels = Some(vec![Label::With((label, value))]),
        }
        self
    }

    /// Append a label matcher to the set of matchers of `Selector` that
    /// selects labels that do not match the provided string.<br>
    /// PromQL example: `http_requests_total{job!="apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("http_requests_total")
    ///     .unwrap()
    ///     .without("job", "apiserver")
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    pub fn without(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Without((label, value))),
            None => self.labels = Some(vec![Label::Without((label, value))]),
        }
        self
    }

    /// Append a label matcher to the set of matchers of `Selector` that
    /// selects labels that regex-match the provided string.
    /// PromQL example: `http_requests_total{job=~"apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("http_requests_total")
    ///     .unwrap()
    ///     .regex_match("job", "apiserver")
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    pub fn regex_match(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Matches((label, value))),
            None => self.labels = Some(vec![Label::Matches((label, value))]),
        }
        self
    }

    /// Append a label matcher to the set of matchers of `Selector` that
    /// selects labels that do not regex-match the provided string.<br>
    /// PromQL example: `http_requests_total{job!~"apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("http_requests_total")
    ///     .unwrap()
    ///     .no_regex_match("job", "apiserver")
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    pub fn no_regex_match(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Clashes((label, value))),
            None => self.labels = Some(vec![Label::Clashes((label, value))]),
        }
        self
    }

    /// Add a time duration to this `Selector`.<br>
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations)
    /// for the correct time duration syntax.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").unwrap().range("1m30s");
    ///
    /// assert!(s.is_ok());
    /// ```
    ///
    /// Providing invalid time durations will lead to an error.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").unwrap().range("30s1m");
    ///
    /// assert!(s.is_err());
    /// ```
    ///
    pub fn range(mut self, duration: &'a str) -> Result<Self, Error> {
        if duration.is_empty() {
            return Err(Error::InvalidTimeDuration);
        }

        validate_duration(&duration)?;

        self.duration = Some(duration);

        Ok(self)
    }
}

impl<'a> fmt::Display for Selector<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut result = String::new();

        if let Some(l) = &self.labels {
            let label_str = l
                .iter()
                .map(|x| match x {
                    Label::With(pair) => format!("{}=\"{}\"", pair.0, pair.1),
                    Label::Without(pair) => format!("{}!=\"{}\"", pair.0, pair.1),
                    Label::Matches(pair) => format!("{}=~\"{}\"", pair.0, pair.1),
                    Label::Clashes(pair) => format!("{}!~\"{}\"", pair.0, pair.1),
                })
                .collect::<Vec<String>>()
                .as_slice()
                .join(",");

            result.push_str(&label_str);
            result.insert(0, '{');
            result.push('}');
        }

        if let Some(m) = self.metric {
            result.insert_str(0, &m);
        }

        if let Some(d) = self.duration {
            let duration = format!("[{}]", d);
            result.push_str(&duration);
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Label;
    use crate::vector::*;
    use std::convert::TryInto;

    #[test]
    fn test_selector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test");

        let result = Selector {
            metric: Some("http_requests_total"),
            labels: Some(vec![
                Label::With(("handler", "/api/comments")),
                Label::Matches(("job", ".*server")),
                Label::Clashes(("status", "4..")),
                Label::Without(("env", "test")),
            ]),
            duration: None,
        };

        assert_eq!(s, result);
    }

    #[test]
    fn test_selector_display_implementation_1() {
        let s = Selector {
            metric: Some("http_requests_total"),
            labels: Some(vec![
                Label::With(("handler", "/api/comments")),
                Label::Matches(("job", ".*server")),
                Label::Clashes(("status", "4..")),
                Label::Without(("env", "test")),
            ]),
            duration: None,
        };

        let result = String::from("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}");

        assert_eq!(s.to_string(), result);
    }

    #[test]
    fn test_selector_display_implementation_2() {
        let s = Selector {
            metric: Some("http_requests_total"),
            labels: Some(vec![
                Label::With(("handler", "/api/comments")),
                Label::Matches(("job", ".*server")),
                Label::Clashes(("status", "4..")),
                Label::Without(("env", "test")),
            ]),
            duration: Some("1m30s"),
        };

        let result = String::from("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}[1m30s]");

        assert_eq!(s.to_string(), result);
    }

    #[test]
    fn test_instant_vector_creation() {
        let v: InstantVector = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .try_into()
            .unwrap();

        let result = InstantVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}".to_string());

        assert_eq!(v, result);
    }

    #[test]
    fn test_range_vector_creation() {
        let v: RangeVector = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .range("5m")
            .unwrap()
            .try_into()
            .unwrap();

        let result = RangeVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}[5m]".to_string());

        assert_eq!(v, result);
    }

    #[test]
    fn test_selector_range_for_error() {
        let s = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .range("");

        assert!(s.is_err());
    }
}
