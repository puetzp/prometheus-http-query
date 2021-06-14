use crate::error::Error;
use crate::util::*;

#[derive(Debug, PartialEq)]
pub struct Selector<'a> {
    metric: Option<&'a str>,
    labels: Option<Vec<Label<'a>>>,
}

impl<'a> Selector<'a> {
    pub fn new() -> Self {
        Selector {
            metric: None,
            labels: None,
        }
    }

    pub fn metric(mut self, metric: &'a str) -> Self
    where
        Self: Sized,
    {
        self.metric = Some(metric);
        self
    }

    pub fn with_label(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::With((label, value))),
            None => self.labels = Some(vec![Label::With((label, value))]),
        }
        self
    }

    pub fn without_label(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Without((label, value))),
            None => self.labels = Some(vec![Label::Without((label, value))]),
        }
        self
    }

    pub fn match_label(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Matches((label, value))),
            None => self.labels = Some(vec![Label::Matches((label, value))]),
        }
        self
    }

    pub fn no_match_label(mut self, label: &'a str, value: &'a str) -> Self
    where
        Self: Sized,
    {
        match self.labels {
            Some(ref mut vec) => vec.push(Label::Clashes((label, value))),
            None => self.labels = Some(vec![Label::Clashes((label, value))]),
        }
        self
    }

    pub fn to_instant_selector(self) -> Result<InstantVector, Error> {
        let selector_str = build_selector_string(self)?;
        Ok(InstantVector(selector_str))
    }

    pub fn to_range_selector(self, duration: &'a str) -> Result<RangeVector, Error> {
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
            .with_label("handler", "/api/comments")
            .match_label("job", ".*server")
            .no_match_label("status", "4..")
            .without_label("env", "test");

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
    /*
    #[test]
    fn test_build_selector_string_for_error_1() {
        let s = Selector {
            metric: None,
            labels: None,
        };

        assert_eq!(
            build_selector_string(s).unwrap_err(),
            Error::IllegalTimeSeriesSelector
        );
    }

    #[test]
    fn test_build_selector_string_for_error_2() {
        let s = Selector {
            metric: Some("group_left"),
            labels: None,
        };

        assert_eq!(
            build_selector_string(s).unwrap_err(),
            Error::IllegalMetricName
        );
    }
    */
    #[test]
    fn test_instant_vector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with_label("handler", "/api/comments")
            .match_label("job", ".*server")
            .no_match_label("status", "4..")
            .without_label("env", "test")
            .to_instant_selector()
            .unwrap();

        let result = InstantVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}".to_string());

        assert_eq!(s, result);
    }

    #[test]
    fn test_range_vector_creation() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with_label("handler", "/api/comments")
            .match_label("job", ".*server")
            .no_match_label("status", "4..")
            .without_label("env", "test")
            .to_range_selector("5m")
            .unwrap();

        let result = RangeVector("http_requests_total{handler=\"/api/comments\",job=~\".*server\",status!~\"4..\",env!=\"test\"}[5m]".to_string());

        assert_eq!(s, result);
    }
    /*
    #[test]
    fn test_range_vector_creation_for_error() {
        let s = Selector::new()
            .metric("http_requests_total")
            .with_label("handler", "/api/comments")
            .match_label("job", ".*server")
            .no_match_label("status", "4..")
            .without_label("env", "test")
            .to_range_selector("")
            .unwrap_err();

        assert_eq!(s, Error::EmptyRange);
    }
     */
}
