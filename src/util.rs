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
pub(crate) enum Unit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Years,
}

// This basically does the same as the Go implementation in
// https://github.com/prometheus/common/blob/00591a3ea9c0d18f6bc983818a23901d4154077f/model/time.go#L190
// However there is no reason to store the duration in another type
// as it is posted to the HTTP API as-is anyway.
// Thus the duration string is only validated.
pub(crate) fn validate_duration(mut duration: &str, allow_negative: bool) -> Result<(), Error> {
    if duration.is_empty() {
        return Err(Error::InvalidTimeDuration);
    }

    if allow_negative {
        duration = duration.strip_prefix('-').unwrap_or(duration);
    }

    // Remember the last encountered unit to check for duplicates and proper ordering.
    // Actually checking for proper ordering is sufficient here because the following
    // unit must be lesser than the leading unit, i.e. the leading unit must be
    // greater or equal the following unit.
    let mut last_unit: Option<Unit> = None;

    // In the go implementation the whole duration string is converted to an
    // time.Duration that is constructed from an int64. Thus the total number
    // of nanoseconds (when each unit is converted to nanoseconds) may not exceed
    // i64::MAX.
    let mut total_nanos: i64 = 0;

    // Each unit is converted to nanoseconds. As "ms" is the most precise unit
    // that can be used, we need to multiply _every_ unit, "ms" and above, by
    // this amount to convert it to nanoseconds.
    const MULTIPLIER: i64 = 1000 * 1000;

    // Add each number character to a string until a unit character is encountered.
    // This string is then cleared to process the next number + unit.
    let mut raw_num = String::new();

    // Iterate the duration string, convert each unit to nanoseconds and add
    // it to the total.
    let mut duration_iter = duration.chars().peekable();
    while let Some(item) = duration_iter.next() {
        if ('0'..='9').contains(&item) {
            raw_num.push(item);
            continue;
        }

        // If the next character is not in 0..=9, it must be a unit. However
        // a unit must be preceded by a number, so at least one index must
        // be stored.
        if raw_num.is_empty() {
            return Err(Error::InvalidTimeDuration);
        }

        let num = raw_num
            .parse::<i64>()
            .map_err(|_| Error::InvalidTimeDuration)?;

        let unit = match item {
            'y' => {
                total_nanos = num
                    .checked_mul(1000 * 60 * 60 * 24 * 365 * MULTIPLIER)
                    .and_then(|n| total_nanos.checked_add(n))
                    .ok_or(Error::InvalidTimeDuration)?;
                Unit::Years
            }
            'w' => {
                total_nanos = num
                    .checked_mul(1000 * 60 * 60 * 24 * 7 * MULTIPLIER)
                    .and_then(|n| total_nanos.checked_add(n))
                    .ok_or(Error::InvalidTimeDuration)?;
                Unit::Weeks
            }
            'd' => {
                total_nanos = num
                    .checked_mul(1000 * 60 * 60 * 24 * MULTIPLIER)
                    .and_then(|n| total_nanos.checked_add(n))
                    .ok_or(Error::InvalidTimeDuration)?;
                Unit::Days
            }
            'h' => {
                total_nanos = num
                    .checked_mul(1000 * 60 * 60 * MULTIPLIER)
                    .and_then(|n| total_nanos.checked_add(n))
                    .ok_or(Error::InvalidTimeDuration)?;
                Unit::Hours
            }
            'm' => {
                if duration_iter.next_if(|&c| c == 's').is_some() {
                    total_nanos = num
                        .checked_mul(1000 * 60 * 60 * MULTIPLIER)
                        .and_then(|n| total_nanos.checked_add(n))
                        .ok_or(Error::InvalidTimeDuration)?;
                    Unit::Milliseconds
                } else {
                    total_nanos = num
                        .checked_mul(1000 * 60 * MULTIPLIER)
                        .and_then(|n| total_nanos.checked_add(n))
                        .ok_or(Error::InvalidTimeDuration)?;
                    Unit::Minutes
                }
            }
            's' => {
                total_nanos = num
                    .checked_mul(1000 * MULTIPLIER)
                    .and_then(|n| total_nanos.checked_add(n))
                    .ok_or(Error::InvalidTimeDuration)?;
                Unit::Seconds
            }
            _ => return Err(Error::InvalidTimeDuration),
        };

        raw_num.clear();

        // Check for duplicates and ordering.
        if matches!(last_unit, Some(x) if x <= unit) {
            println!("matches");
            return Err(Error::InvalidTimeDuration);
        } else {
            last_unit = Some(unit);
        }
    }

    // When the whole duration string has been iterated over, no more
    // indices should be left.
    if !raw_num.is_empty() {
        return Err(Error::InvalidTimeDuration);
    }

    if total_nanos < 0 {
        return Err(Error::InvalidTimeDuration);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_duration() {
        // Large duration, but still in range.
        let input = "292y";
        assert!(validate_duration(input, false).is_ok());

        // Large duration, but still in range.
        let input = "9223372036s";
        assert!(validate_duration(input, false).is_ok());

        // Out of range (greater than i64::MAX when converted to ns).
        let input = "293y";
        assert!(validate_duration(input, false).is_err());

        // Out of range (greater than i64::MAX when converted to ns).
        let input = "9223372037s";
        assert!(validate_duration(input, false).is_err());

        // Normal range with multiple units and in proper order.
        let input = "2y5m30s";
        assert!(validate_duration(input, false).is_ok());

        // Normal range with multiple units and in proper order.
        let input = "2y5m30s15ms";
        assert!(validate_duration(input, false).is_ok());

        // Same as the prior but negative.
        let input = "-2y5m30s";
        assert!(validate_duration(input, true).is_ok());

        // Same as the prior but negative is not allowed.
        let input = "-2y5m30s";
        assert!(validate_duration(input, false).is_err());

        // Only exactly one minus is stripped.
        let input = "--2y5m30s";
        assert!(validate_duration(input, true).is_err());

        // Wrong order.
        let input = "2y5m1h30s";
        assert!(validate_duration(input, false).is_err());

        // Duplicate.
        let input = "2y5m30s1s";
        assert!(validate_duration(input, false).is_err());

        // No unit at all.
        let input = "200";
        assert!(validate_duration(input, false).is_err());

        // Missing unit in trailing number.
        let input = "1h30";
        assert!(validate_duration(input, false).is_err());
    }
}
