use crate::selector::Selector;
use crate::util::{Group, Match};
use std::convert::TryFrom;
use std::fmt;

/// An instant vector expression that can be further operated on with functions/aggregations
/// or passed to a [crate::Client] in order to evaluate.
#[derive(Debug, PartialEq)]
pub struct InstantVector(pub String);

impl fmt::Display for InstantVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let InstantVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for InstantVector {
    type Error = crate::error::Error;

    /// Convert a [Selector] to an [InstantVector].
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let v: Result<InstantVector, Error> = Selector::new()
    ///         .metric("some_metric")?
    ///         .try_into();
    ///
    ///     assert!(v.is_ok());
    ///
    ///     Ok(())
    /// }
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
    /// Add one instant vector to another. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.add(two, Some(Match::On(&["some_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} + on (some_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
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

    /// Subtract one instant vector from another. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///      let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.subtract(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} - ignoring (other_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
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

    /// Multiply one instant vector by another. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.multiply(two, Some(Match::On(&["some_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} * on (some_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
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

    /// Divide one instant vector by another. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.divide(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} / ignoring (other_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
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

    /// Divide one instant vector by another with remainder. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.modulo(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} % ignoring (other_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
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

    /// Exponentiate one instant vector by another. Additional modifiers ([Match] and [Group])
    /// can be used to alter the matching behaviour between two instant vectors.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#arithmetic-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Match, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.power(two, Some(Match::Ignoring(&["other_label"])), None);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} ^ ignoring (other_label) other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected);
    ///
    ///     Ok(())
    /// }
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

    /// Intersect two vectors so that the result vector consists of all elements of vector1
    /// for which there are matching elements in vector2.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#logical-set-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.and(two);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} and other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn and(self, other: InstantVector) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" and");

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Combine two vectors so that the result vector consists of all elements of vector1
    /// and also all elements of vector2 which do not have matching label sets in vector1.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#logical-set-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.or(two);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} or other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn or(self, other: InstantVector) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" or");

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Combine two vectors so that the result vector consists only of those elements of
    /// vector1 for which there are no matching elements in vector2.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#logical-set-binary-operators)
    /// for details on this topic.
    ///
    /// ```rust
    /// use prometheus_http_query::{Selector, InstantVector, Error};
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.unless(two);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} unless other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn unless(self, other: InstantVector) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" unless");

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `==` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.eq_vector(two, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} == other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn eq_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" ==");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `!=` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.ne_vector(two, true);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} != bool other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn ne_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" !=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `>` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.gt_vector(two, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} > other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn gt_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" >");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `<` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.lt_vector(two, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} < other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lt_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" <");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `>=` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.ge_vector(two, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} >= other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn ge_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" >=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `<=` operator to two vectors. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let two: InstantVector = Selector::new()
    ///         .metric("other_metric")?
    ///         .with("some_label", "some_value")
    ///         .with("other_label", "other_value")
    ///         .try_into()?;
    ///
    ///     let new = one.le_vector(two, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} <= other_metric{some_label="some_value",other_label="other_value"}"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn le_vector(self, other: InstantVector, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;
        let InstantVector(other) = other;

        this.push_str(" <=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", other));

        InstantVector(this)
    }

    /// Apply the `==` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.eq_scalar(8.5, true);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} == bool 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn eq_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" ==");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

        InstantVector(this)
    }

    /// Apply the `!=` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.ne_scalar(8.5, true);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} != bool 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn ne_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" !=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

        InstantVector(this)
    }

    /// Apply the `>` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.gt_scalar(8.5, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} > 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn gt_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" >");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

        InstantVector(this)
    }

    /// Apply the `<` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.lt_scalar(8.5, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} < 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lt_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" <");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

        InstantVector(this)
    }

    /// Apply the `>=` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.ge_scalar(8.5, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} >= 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn ge_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" >=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

        InstantVector(this)
    }

    /// Apply the `<=` operator to a vector and a scalar. Optionally set the `bool` parameter
    /// to modify the query result as per the PromQL documentation.
    /// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/#comparison-binary-operators)
    /// for details on comparison binary operators.
    ///
    /// ```rust
    /// use prometheus_http_query::Selector;
    /// use prometheus_http_query::InstantVector;
    /// use std::convert::TryInto;
    ///
    /// fn main() -> Result<(), prometheus_http_query::Error> {
    ///     let one: InstantVector = Selector::new()
    ///         .metric("some_metric")?
    ///         .with("some_label", "some_value")
    ///         .try_into()?;
    ///
    ///     let new = one.le_scalar(8.5, false);
    ///
    ///     // This would ultimately be the query string posted to the HTTP API.
    ///     let expected = r#"some_metric{some_label="some_value"} <= 8.5"#;
    ///
    ///     assert_eq!(new.to_string(), expected.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn le_scalar(self, scalar: f64, return_bool: bool) -> Self {
        let InstantVector(mut this) = self;

        this.push_str(" <=");

        if return_bool {
            this.push_str(" bool");
        }

        this.push_str(&format!(" {}", scalar));

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

/// An range vector expression that can be further operated on with functions/aggregations
/// or passed to a `Client` in order to evaluate.
#[derive(Debug, PartialEq)]
pub struct RangeVector(pub String);

impl fmt::Display for RangeVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let RangeVector(s) = self;
        write!(f, "{}", s)
    }
}

impl TryFrom<Selector<'_>> for RangeVector {
    type Error = crate::error::Error;

    /// Convert a [Selector] to a [RangeVector].
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

        if selector.range.is_none() {
            return Err(crate::error::Error::InvalidRangeVector);
        };

        Ok(RangeVector(selector.to_string()))
    }
}
