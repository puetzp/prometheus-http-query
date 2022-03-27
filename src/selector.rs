use crate::util::*;
use std::fmt;

/// A time series selector that is gradually built from a metric name and/or
/// a set of label matchers.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector<'a> {
    pub(crate) labels: Vec<Label<'a>>,
}

impl<'a> Default for Selector<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Selector<'a> {
    /// Create a new instance of [Selector].
    pub fn new() -> Self {
        Selector { labels: vec![] }
    }

    /// Select a metric name for this [Selector].
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let select = Selector::new().metric("http_requests_total");
    ///
    /// // This is equal to:
    /// let other_select = Selector::new().eq("__name__", "http_requests_total");
    ///
    /// assert_eq!(select, other_select);
    /// ```
    pub fn metric(mut self, metric: &'a str) -> Self
    where
        Self: Sized,
    {
        self.labels.push(Label::Equal(("__name__", metric)));
        self
    }

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job="apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let select = Selector::new()
    ///     .metric("http_requests_total")
    ///     .eq("job", "apiserver")
    ///     .to_string();
    ///
    /// let expected = r#"{__name__="http_requests_total",job="apiserver"}"#.to_string();
    ///
    /// assert_eq!(select, expected);
    /// ```
    pub fn eq(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        self.labels.push(Label::Equal((label, value)));
        self
    }

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that do not match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job!="apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let select = Selector::new()
    ///     .metric("http_requests_total")
    ///     .ne("job", "apiserver")
    ///     .to_string();
    ///
    /// let expected = r#"{__name__="http_requests_total",job!="apiserver"}"#.to_string();
    ///
    /// assert_eq!(select, expected);
    /// ```
    pub fn ne(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        self.labels.push(Label::NotEqual((label, value)));
        self
    }

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that regex-match the provided string.
    /// PromQL equivalent: `http_requests_total{job=~"apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let select = Selector::new()
    ///     .metric("http_requests_total")
    ///     .regex_eq("job", "apiserver")
    ///     .to_string();
    ///
    /// let expected = r#"{__name__="http_requests_total",job=~"apiserver"}"#.to_string();
    ///
    /// assert_eq!(select, expected);
    /// ```
    pub fn regex_eq(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        self.labels.push(Label::RegexEqual((label, value)));
        self
    }

    /// Append a label matcher to the set of matchers of [Selector] that
    /// selects labels that do not regex-match the provided string.<br>
    /// PromQL equivalent: `http_requests_total{job!~"apiserver"}`
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    ///
    /// let select = Selector::new()
    ///     .metric("http_requests_total")
    ///     .regex_ne("job", "apiserver")
    ///     .to_string();
    ///
    /// let expected = r#"{__name__="http_requests_total",job!~"apiserver"}"#.to_string();
    ///
    /// assert_eq!(select, expected);
    /// ```
    pub fn regex_ne(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        self.labels.push(Label::RegexNotEqual((label, value)));
        self
    }
}

impl<'a> fmt::Display for Selector<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let matchers = self
            .labels
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        write!(f, "{{{}}}", matchers.as_slice().join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Label;

    #[test]
    fn test_selector_display_impl() {
        let s = Selector {
            labels: vec![
                Label::Equal(("__name__", "http_requests_total")),
                Label::Equal(("handler", "/api/comments")),
                Label::RegexEqual(("job", ".*server")),
                Label::RegexNotEqual(("status", "4..")),
                Label::NotEqual(("env", "test")),
            ],
        };

        let result = String::from("{__name__=\"http_requests_total\",handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}");

        assert_eq!(s.to_string(), result);
    }
}
