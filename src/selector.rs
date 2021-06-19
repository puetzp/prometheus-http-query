use crate::error::Error;
use crate::util::*;
use std::fmt;

/// A time series selector that is gradually built from a metric name and/or
/// a set of label matchers.
///
/// For final validation and further processing the selector is then
/// converted to either a [crate::InstantVector] or [crate::RangeVector].
///
#[derive(Debug, Clone, PartialEq)]
pub struct Selector<'a> {
    pub(crate) metric: Option<&'a str>,
    pub(crate) labels: Option<Vec<Label<'a>>>,
    pub(crate) range: Option<&'a str>,
    pub(crate) offset: Option<&'a str>,
    pub(crate) at_modifier: Option<i64>,
}

impl<'a> Default for Selector<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Selector<'a> {
    /// Simply return an empty [Selector] to build on.
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
            range: None,
            offset: None,
            at_modifier: None,
        }
    }

    /// Select a metric name for this [Selector].
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

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job="apiserver"}`
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

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that do not match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job!="apiserver"}`
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

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that regex-match the provided string.
    /// PromQL equivalent: `http_requests_total{job=~"apiserver"}`
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

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that do not regex-match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job!~"apiserver"}`
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

    /// Add a time range to this [Selector] (effectively priming this [Selector] to be converted to a [crate::RangeVector]).<br>
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

        self.range = Some(duration);

        Ok(self)
    }

    /// Add a time offset to this [Selector].<br>
    /// See the Prometheus reference regarding [time durations](https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations)
    /// and [offsets](https://prometheus.io/docs/prometheus/latest/querying/basics/#offset-modifier)
    /// for the correct time duration syntax.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").unwrap().offset("1m30s");
    ///
    /// assert!(s.is_ok());
    /// ```
    ///
    /// Providing invalid time durations will lead to an error.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").unwrap().offset("30s1m");
    ///
    /// assert!(s.is_err());
    /// ```
    ///
    pub fn offset(mut self, duration: &'a str) -> Result<Self, Error> {
        if duration.is_empty() {
            return Err(Error::InvalidTimeDuration);
        }

        validate_duration(&duration)?;

        self.offset = Some(duration);

        Ok(self)
    }

    /// Add a @ modifier to this [Selector].<br>
    /// See the Prometheus reference regarding [time durations](https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations)
    /// and [@ modifiers](https://prometheus.io/docs/prometheus/latest/querying/basics/#modifier)
    /// for details.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector};
    /// use std::convert::TryInto;
    ///
    /// let s: Result<InstantVector, _> = Selector::new().metric("some_metric").unwrap().at(1623855855).try_into();
    ///
    /// assert!(s.is_ok());
    /// ```
    pub fn at(mut self, time: i64) -> Self {
        self.at_modifier = Some(time);
        self
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

        if let Some(r) = self.range {
            let range = format!("[{}]", r);
            result.push_str(&range);
        }

        if let Some(o) = self.offset {
            let offset = format!(" offset {}", o);
            result.push_str(&offset);
        }

        if let Some(a) = self.at_modifier {
            let at_mod = format!(" @ {}", a);
            result.push_str(&at_mod);
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
            range: None,
            offset: None,
            at_modifier: None,
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
            range: None,
            offset: None,
            at_modifier: None,
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
            range: Some("1m30s"),
            offset: None,
            at_modifier: None,
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
    fn test_instant_vector_creation_with_offset() {
        let v: InstantVector = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .offset("1w")
            .unwrap()
            .try_into()
            .unwrap();

        let result = InstantVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"} offset 1w".to_string());

        assert_eq!(v, result);
    }

    #[test]
    fn test_instant_vector_creation_with_offset_and_at_modifier() {
        let v: InstantVector = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .offset("1w")
            .unwrap()
            .at(1623855625)
            .try_into()
            .unwrap();

        let result = InstantVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"} offset 1w @ 1623855625".to_string());

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
    fn test_range_vector_creation_with_offset() {
        let v: RangeVector = Selector::new()
            .metric("http_requests_total")
            .unwrap()
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .range("5m")
            .unwrap()
            .offset("-1y")
            .unwrap()
            .try_into()
            .unwrap();

        let result = RangeVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}[5m] offset -1y".to_string());

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
