use std::fmt;

// TODO: count_values, bottomk, topk, quantile
#[derive(Debug)]
pub(crate) enum Aggregation<'a> {
    Sum(Option<LabelList<'a>>),
    Min(Option<LabelList<'a>>),
    Max(Option<LabelList<'a>>),
    Avg(Option<LabelList<'a>>),
    Group(Option<LabelList<'a>>),
    Stddev(Option<LabelList<'a>>),
    Stdvar(Option<LabelList<'a>>),
    Count(Option<LabelList<'a>>),
    CountValues(Option<LabelList<'a>>, &'a str),
    BottomK(Option<LabelList<'a>>, usize),
    TopK(Option<LabelList<'a>>, usize),
    Quantile(Option<LabelList<'a>>, f32),
}

impl<'a> fmt::Display for Aggregation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Aggregation::Sum(labels) => match labels {
                Some(l) => write!(f, "sum {} (", l.to_string()),
                None => write!(f, "sum ("),
            },
            Aggregation::Min(labels) => match labels {
                Some(l) => write!(f, "min {} (", l.to_string()),
                None => write!(f, "min ("),
            },
            Aggregation::Max(labels) => match labels {
                Some(l) => write!(f, "max {} (", l.to_string()),
                None => write!(f, "max ("),
            },
            Aggregation::Avg(labels) => match labels {
                Some(l) => write!(f, "avg {} (", l.to_string()),
                None => write!(f, "avg ("),
            },
            Aggregation::Group(labels) => match labels {
                Some(l) => write!(f, "group {} (", l.to_string()),
                None => write!(f, "group ("),
            },
            Aggregation::Stddev(labels) => match labels {
                Some(l) => write!(f, "stddev {} (", l.to_string()),
                None => write!(f, "stddev ("),
            },
            Aggregation::Stdvar(labels) => match labels {
                Some(l) => write!(f, "stdvar {} (", l.to_string()),
                None => write!(f, "stdvar ("),
            },
            Aggregation::Count(labels) => match labels {
                Some(l) => write!(f, "count {} (", l.to_string()),
                None => write!(f, "count ("),
            },
            Aggregation::CountValues(labels, parameter) => match labels {
                Some(l) => write!(f, "count_values {} ({},", l.to_string(), parameter),
                None => write!(f, "count_values ({},", parameter),
            },
            Aggregation::BottomK(labels, parameter) => match labels {
                Some(l) => write!(f, "bottomk {} ({},", l.to_string(), parameter),
                None => write!(f, "bottomk ({},", parameter),
            },
            Aggregation::TopK(labels, parameter) => match labels {
                Some(l) => write!(f, "topk {} ({},", l.to_string(), parameter),
                None => write!(f, "topk ({},", parameter),
            },
            Aggregation::Quantile(labels, parameter) => match labels {
                Some(l) => write!(f, "quantile {} ({},", l.to_string(), parameter),
                None => write!(f, "quantile ({},", parameter),
            },
        }
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
