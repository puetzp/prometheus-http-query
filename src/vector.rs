use crate::selector::Selector;
use crate::util::{Group, Match};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InstantVector(pub(crate) String);

impl fmt::Display for InstantVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let InstantVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for InstantVector {
    type Error = crate::error::Error;

    /// Convert a `Selector` to an `InstantVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<InstantVector, Error> = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    fn try_from(selector: Selector) -> Result<InstantVector, Self::Error> {
        if selector.labels.is_none() && selector.metric.is_none() {
            return Err(crate::error::Error::IllegalTimeSeriesSelector);
        }

        let selector_str = selector.to_string();
        Ok(InstantVector(selector_str))
    }
}

impl InstantVector {
    /// Add one instant vector to another. Additional modifiers (`Match` and `Group`)
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Match;
    /// use std::convert::TryInto;
    ///
    /// let one: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let two: InstantVector = Selector::new()
    ///     .metric("other_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .with("other_label", "other_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let new = one.add(two, Some(Match::On(&["some_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} + on (some_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn add(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> InstantVector {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" +");

        if let Some(labels) = match_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        if let Some(labels) = group_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Subtract one instant vector from another. Additional modifiers (`Match` and `Group`)
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use prometheus_http_query::Match;
    /// use std::convert::TryInto;
    ///
    /// let one: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let two: InstantVector = Selector::new()
    ///     .metric("other_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .with("other_label", "other_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let new = one.subtract(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} - ignoring (other_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn subtract(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> InstantVector {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" -");

        if let Some(labels) = match_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        if let Some(labels) = group_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeVector(pub(crate) String);

impl fmt::Display for RangeVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let RangeVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for RangeVector {
    type Error = crate::error::Error;

    /// Convert a `Selector` to an `RangeVector`.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::RangeVector;
    /// use prometheus_http_query::Error;
    /// use std::convert::TryInto;
    ///
    /// let v: Result<RangeVector, Error> = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .range("1m30s")
    ///     .unwrap()
    ///     .try_into();
    ///
    /// assert!(v.is_ok());
    /// ```
    fn try_from(selector: Selector) -> Result<RangeVector, Self::Error> {
        if selector.labels.is_none() && selector.metric.is_none() {
            return Err(crate::error::Error::IllegalTimeSeriesSelector);
        }

        if selector.duration.is_none() {
            return Err(crate::error::Error::InvalidRangeVector);
        };

        Ok(RangeVector(selector.to_string()))
    }
}
