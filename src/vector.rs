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
    fn try_from(selector: Selector) -> Result<Self, Self::Error> {
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
    ) -> Self {
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
    ) -> Self {
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

    /// Multiply one instant vector by another. Additional modifiers (`Match` and `Group`)
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
    /// let new = one.multiply(two, Some(Match::On(&["some_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} * on (some_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn multiply(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" *");

        if let Some(labels) = match_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        if let Some(labels) = group_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Divide one instant vector by another. Additional modifiers (`Match` and `Group`)
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
    /// let new = one.divide(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} / ignoring (other_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn divide(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" /");

        if let Some(labels) = match_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        if let Some(labels) = group_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Divide one instant vector by another with remainder. Additional modifiers (`Match` and `Group`)
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
    /// let new = one.modulo(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} % ignoring (other_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn modulo(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" %");

        if let Some(labels) = match_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        if let Some(labels) = group_modifier {
            this.push_str(&format!(" {}", labels.to_string()));
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Exponentiate one instant vector by another. Additional modifiers (`Match` and `Group`)
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
    /// let new = one.power(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    /// let expected = String::from("some_metric{some_label=\"some_value\"} ^ ignoring (other_label) other_metric{some_label=\"some_value\",other_label=\"other_value\"}");
    ///
    /// assert_eq!(new.to_string(), expected);
    /// ```
    pub fn power(
        self,
        other: InstantVector,
        match_modifier: Option<Match>,
        group_modifier: Option<Group>,
    ) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" ^");

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

impl std::ops::Add<f64> for InstantVector {
    type Output = Self;

    /// Add a scalar value to every data sample in a vector.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v + 4.0;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} + 4"));
    fn add(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" + {}", scalar.to_string()));
        InstantVector(vec)
    }
}

impl std::ops::Sub<f64> for InstantVector {
    type Output = Self;

    /// Subtract a scalar value from every data sample in a vector.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v - 4.5;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} - 4.5"));
    fn sub(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" - {}", scalar.to_string()));
        InstantVector(vec)
    }
}

impl std::ops::Mul<f64> for InstantVector {
    type Output = Self;

    /// Multiply each data sample in a vector by a scalar value.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v * 2.0;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} * 2"));
    fn mul(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" * {}", scalar.to_string()));
        InstantVector(vec)
    }
}

impl std::ops::Div<f64> for InstantVector {
    type Output = Self;

    /// Divide each data sample in a vector by a scalar value.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v / 2.0;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} / 2"));
    fn div(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" / {}", scalar.to_string()));
        InstantVector(vec)
    }
}

impl std::ops::Rem<f64> for InstantVector {
    type Output = Self;

    /// Divide each data sample in a vector by a scalar value
    /// with remainder.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v % 2.0;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} % 2"));
    fn rem(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" % {}", scalar.to_string()));
        InstantVector(vec)
    }
}

impl std::ops::BitXor<f64> for InstantVector {
    type Output = Self;

    /// Exponentiate each data sample in a vector by a scalar value.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// let v: InstantVector = Selector::new()
    ///     .metric("some_metric")
    ///     .unwrap()
    ///     .with("some_label", "some_value")
    ///     .try_into()
    ///     .unwrap();
    ///
    /// let v = v ^ 2.0;
    ///
    /// assert_eq!(v.to_string(), String::from("some_metric{some_label=\"some_value\"} ^ 2"));
    fn bitxor(self, scalar: f64) -> Self {
        let InstantVector(mut vec) = self;
        vec.push_str(&format!(" ^ {}", scalar.to_string()));
        InstantVector(vec)
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
