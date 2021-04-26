use std::fmt;

// TODO: count_values, bottomk, topk, quantile
#[derive(Debug)]
pub(crate) enum Aggregation<'a> {
    Sum(Option<LabelList<'a>>),
    Min(Option<LabelList<'a>>),
    Max(Option<LabelList<'a>>),
    Group(Option<LabelList<'a>>),
    Stddev(Option<LabelList<'a>>),
    Stdvar(Option<LabelList<'a>>),
    Count(Option<LabelList<'a>>),
}

#[derive(Debug)]
pub(crate) enum LabelList<'a> {
    By(&'a [&'a str]),
    Without(&'a [&'a str]),
}

#[derive(Debug)]
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
