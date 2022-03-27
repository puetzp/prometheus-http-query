use serde::Deserialize;
use std::fmt;

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
            Label::Equal((k, v)) => write!(f, "{}=\"{}\"", k, v),
            Label::NotEqual((k, v)) => write!(f, "{}!=\"{}\"", k, v),
            Label::RegexEqual((k, v)) => write!(f, "{}=~\"{}\"", k, v),
            Label::RegexNotEqual((k, v)) => write!(f, "{}!~\"{}\"", k, v),
        }
    }
}
