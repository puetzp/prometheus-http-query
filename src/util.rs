use crate::error::Error;
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
pub(crate) enum Label<'c> {
    With((&'c str, &'c str)),
    Without((&'c str, &'c str)),
    Matches((&'c str, &'c str)),
    Clashes((&'c str, &'c str)),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum Duration {
    Years(usize),
    Weeks(usize),
    Days(usize),
    Hours(usize),
    Minutes(usize),
    Seconds(usize),
    Milliseconds(usize),
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Years(d) => write!(f, "{}y", d),
            Duration::Weeks(d) => write!(f, "{}w", d),
            Duration::Days(d) => write!(f, "{}d", d),
            Duration::Hours(d) => write!(f, "{}h", d),
            Duration::Minutes(d) => write!(f, "{}m", d),
            Duration::Seconds(d) => write!(f, "{}s", d),
            Duration::Milliseconds(d) => write!(f, "{}ms", d),
        }
    }
}

pub(crate) fn validate_duration(duration: &str) -> Result<(), Error> {
    let raw_duration = duration.trim_start_matches('-');

    let chars = ['s', 'm', 'h', 'd', 'w', 'y'];

    let raw_durations: Vec<&str> = raw_duration
        .split_inclusive(chars.as_ref())
        .map(|s| s.split_inclusive("ms"))
        .flatten()
        .collect();

    let mut durations: Vec<Duration> = vec![];

    for d in raw_durations {
        if d.ends_with("ms") {
            match d.strip_suffix("ms").unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Milliseconds(num);

                    let predicate = durations
                        .iter()
                        .any(|x| matches!(x, Duration::Milliseconds(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('s') {
            match d.strip_suffix('s').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Seconds(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Seconds(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('m') {
            match d.strip_suffix('m').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Minutes(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Minutes(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('h') {
            match d.strip_suffix('h').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Hours(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Hours(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('d') {
            match d.strip_suffix('d').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Days(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Days(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('w') {
            match d.strip_suffix('w').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Weeks(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Weeks(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else if d.ends_with('y') {
            match d.strip_suffix('y').unwrap().parse::<usize>() {
                Ok(num) => {
                    let val = Duration::Years(num);

                    let predicate = durations.iter().any(|x| matches!(x, Duration::Years(_)));

                    if !predicate {
                        durations.push(val);
                    } else {
                        return Err(Error::InvalidTimeDuration);
                    }
                }
                Err(_) => return Err(Error::InvalidTimeDuration),
            }
        } else {
            return Err(Error::InvalidTimeDuration);
        }
    }

    let tmp_cpy = durations.clone();

    durations.sort_unstable();

    if tmp_cpy != durations {
        return Err(Error::InvalidTimeDuration);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_duration() {
        let input = "2y5m30s";
        assert_eq!(validate_duration(input).unwrap(), ());
    }
}
