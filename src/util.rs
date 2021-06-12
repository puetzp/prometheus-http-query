use chrono::DateTime;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct InstantVector(pub(crate) String);

impl fmt::Display for InstantVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let InstantVector(s) = self;
        write!(f, "{}", s)
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

#[derive(Debug)]
pub enum LabelList<'a> {
    By(&'a [&'a str]),
    Without(&'a [&'a str]),
}

impl<'a> fmt::Display for LabelList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LabelList::By(list) => write!(f, "by ({})", list.join(",")),
            LabelList::Without(list) => write!(f, "without ({})", list.join(",")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Label<'c> {
    With((&'c str, &'c str)),
    Without((&'c str, &'c str)),
    Matches((&'c str, &'c str)),
    Clashes((&'c str, &'c str)),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Duration {
    Milliseconds(usize),
    Seconds(usize),
    Minutes(usize),
    Hours(usize),
    Days(usize),
    Weeks(usize),
    Years(usize),
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Milliseconds(d) => write!(f, "{}ms", d),
            Duration::Seconds(d) => write!(f, "{}s", d),
            Duration::Minutes(d) => write!(f, "{}m", d),
            Duration::Hours(d) => write!(f, "{}h", d),
            Duration::Days(d) => write!(f, "{}d", d),
            Duration::Weeks(d) => write!(f, "{}w", d),
            Duration::Years(d) => write!(f, "{}y", d),
        }
    }
}

pub(crate) fn validate_timestamp(timestamp: &str) -> bool {
    if f64::from_str(timestamp).is_ok() {
        true
    } else if DateTime::parse_from_rfc3339(timestamp).is_ok() {
        true
    } else {
        false
    }
}
