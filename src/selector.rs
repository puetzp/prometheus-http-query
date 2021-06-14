use crate::error::Error;
use crate::util::*;

/// A time series selector that is built from a metric name and/or
/// a set of label matchers using various methods documented below.
///
/// For final validation and further processing the selector is then
/// converted to either a `InstantVector` or `RangeVector`.
///
#[derive(Debug, PartialEq)]
pub struct Selector<'a> {
    metric: Option<&'a str>,
    labels: Option<Vec<Label<'a>>>,
}

impl<'a> Selector<'a> {
    /// Simply return an empty `Selector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// assert!(Selector::new().to_instant_vector().is_err());
    /// ```
    pub fn new() -> Self {
        Selector {
            metric: None,
            labels: None,
        }
    }

    /// Select a metric name for this `Selector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("http_requests_total").to_instant_vector();
    ///
    /// assert!(s.is_ok());
    /// ```
    ///
    /// ... which must not be any reserved PromQL keyword.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("group_left").to_instant_vector();
    ///
    /// assert!(s.is_err());
    /// ```
    pub fn metric(mut self, metric: &'a str) -> Self
    where
        Self: Sized,
    {
        self.metric = Some(metric);
        self
    }

    /// Append a label matcher to the set of matchers of `Selector` that
    /// selects labels that match the provided string.
    /// Corresponding PromQL example: http_requests_total{job="apiserver"}
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new()
    ///     .metric("http_requests_total")
    ///     .with("job", "apiserver")
    ///     .to_instant_vector();
    ///
    /// assert!(s.is_ok());
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
    /// selects labels that do not match the provided string.
    /// Corresponding PromQL example: http_requests_total{job!="apiserver"}
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new()
    ///     .metric("http_requests_total")
    ///     .without("job", "apiserver")
    ///     .to_instant_vector();
    ///
    /// assert!(s.is_ok());
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
    /// Corresponding PromQL example: http_requests_total{job=~"apiserver"}
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new()
    ///     .metric("http_requests_total")
    ///     .regex_match("job", "apiserver")
    ///     .to_instant_vector();
    ///
    /// assert!(s.is_ok());
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
    /// selects labels that do not regex-match the provided string.
    /// Corresponding PromQL example: http_requests_total{job=~"apiserver"}
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new()
    ///     .metric("http_requests_total")
    ///     .no_regex_match("job", "apiserver")
    ///     .to_instant_vector();
    ///
    /// assert!(s.is_ok());
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

    /// Convert this `Selector` to an `InstantVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").to_instant_vector();
    ///
    /// assert!(s.is_ok());
    /// ```
    pub fn to_instant_vector(self) -> Result<InstantVector, Error> {
        let selector_str = build_selector_string(self)?;
        Ok(InstantVector(selector_str))
    }

    /// Convert this `Selector` to a `RangeVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").to_range_vector("1m30s");
    ///
    /// assert!(s.is_ok());
    /// ```
    ///
    /// ... while invalid time durations will lead to an error.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let s = Selector::new().metric("some_metric").to_range_vector("30s1m");
    ///
    /// assert!(s.is_err());
    /// ```
    ///
    pub fn to_range_vector(self, duration: &'a str) -> Result<RangeVector, Error> {
        if duration.is_empty() {
            return Err(Error::InvalidTimeDuration);
        }

        validate_duration(&duration)?;

        let dur = format!("[{}]", duration);
        let mut selector_str = build_selector_string(self)?;
        selector_str.push_str(&dur);
        Ok(RangeVector(selector_str))
    }
}

fn build_selector_string(selector: Selector) -> Result<String, Error> {
    let labels = match selector.labels {
        Some(l) => {
            let joined = l
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

            Some(joined)
        }
        None => None,
    };

    match selector.metric {
        Some(m) => {
            match m {
                "bool" | "on" | "ignoring" | "group_left" | "group_right" => {
                    return Err(Error::IllegalMetricName)
                }
                _ => {}
            }

            match labels {
                Some(l) => Ok(format!("{}{{{}}}", m, l)),
                None => Ok(m.to_string()),
            }
        }
        None => match labels {
            Some(l) => Ok(format!("{{{}}}", l)),
            None => return Err(Error::IllegalTimeSeriesSelector),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Label;

    #[test]
    fn test_selector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
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
        };

        assert_eq!(s, result);
    }

    #[test]
    fn test_build_selector_string() {
        let s = Selector {
            metric: Some("http_requests_total"),
            labels: Some(vec![
                Label::With(("handler", "/api/comments")),
                Label::Matches(("job", ".*server")),
                Label::Clashes(("status", "4..")),
                Label::Without(("env", "test")),
            ]),
        };

        let result = String::from("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}");

        assert_eq!(build_selector_string(s).unwrap(), result);
    }

    #[test]
    fn test_build_selector_string_for_error_1() {
        let s = Selector {
            metric: None,
            labels: None,
        };

        assert!(build_selector_string(s).is_err());
    }

    #[test]
    fn test_build_selector_string_for_error_2() {
        let s = Selector {
            metric: Some("group_left"),
            labels: None,
        };

        assert!(build_selector_string(s).is_err());
    }

    #[test]
    fn test_instant_vector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .to_instant_vector()
            .unwrap();

        let result = InstantVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}".to_string());

        assert_eq!(s, result);
    }

    #[test]
    fn test_range_vector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .to_range_vector("5m")
            .unwrap();

        let result = RangeVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}[5m]".to_string());

        assert_eq!(s, result);
    }

    #[test]
    fn test_range_vector_creation_for_error() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with("handler", "/api/comments")
            .regex_match("job", ".*server")
            .no_regex_match("status", "4..")
            .without("env", "test")
            .to_range_vector("");

        assert!(s.is_err());
    }
}
