use crate::error::Error;
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

#[derive(Debug, PartialEq)]
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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Milliseconds(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Seconds(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Minutes(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Hours(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Days(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Weeks(_) => true,
                        _ => false,
                    });

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

                    let predicate = durations.iter().any(|x| match x {
                        Duration::Years(_) => true,
                        _ => false,
                    });

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
