use serde::Deserialize;
use std::fmt;

/// A helper type that provides label matching logic for e.g. aggregations like `sum`.<br>
///
/// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/) for details.
#[derive(Debug)]
pub enum Aggregate<'a> {
    By(&'a [&'a str]),
    Without(&'a [&'a str]),
}

impl<'a> fmt::Display for Aggregate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Aggregate::By(list) => write!(f, "by ({})", list.join(",")),
            Aggregate::Without(list) => write!(f, "without ({})", list.join(",")),
        }
    }
}

/// A helper type that provides label matching logic for e.g. binary operations (between instant vectors).<br>
///
/// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/) for details.
#[derive(Debug)]
pub enum Match<'a> {
    On(&'a [&'a str]),
    Ignoring(&'a [&'a str]),
}

impl<'a> fmt::Display for Match<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Match::On(list) => write!(f, "on ({})", list.join(",")),
            Match::Ignoring(list) => write!(f, "ignoring ({})", list.join(",")),
        }
    }
}

/// A helper type that provides grouping logic for e.g. vector matching.<br>
///
/// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/operators/) for details.
#[derive(Debug)]
pub enum Group<'a> {
    Left(&'a [&'a str]),
    Right(&'a [&'a str]),
}

impl<'a> fmt::Display for Group<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Left(list) => write!(f, "group_left ({})", list.join(",")),
            Group::Right(list) => write!(f, "group_right ({})", list.join(",")),
        }
    }
}

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
