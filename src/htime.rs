use std::fmt;
use std::time;

/// A wrapper type that allows to display a Duration
#[derive(Debug, Clone)]
pub struct Duration(time::Duration);

impl From<time::Duration> for Duration {
    fn from(duration: time::Duration) -> Self {
        Self(duration)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_print(f, &self.0)
    }
}

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

fn pretty_print(f: &mut fmt::Formatter<'_>, d: &time::Duration) -> fmt::Result {
    let mut d = d.as_secs();
    let mut first = true;

    for (secs, suffix) in [(DAY, "day"), (HOUR, "hour"), (MINUTE, "minute")] {
        if d < secs {
            continue;
        }

        let units = d / secs;

        if !first {
            f.write_str(" ")?;
        }
        first = false;

        write!(f, "{units} {suffix}")?;
        if units != 1 {
            f.write_str("s")?;
        }

        d %= secs;
    }

    if first {
        f.write_str("less than a minute")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let pp = |secs| Duration::from(time::Duration::from_secs(secs)).to_string();

        assert_eq!(pp(0), "less than a minute");
        assert_eq!(pp(MINUTE - 1), "less than a minute");
        assert_eq!(pp(30), "less than a minute");
        assert_eq!(pp(MINUTE), "1 minute");

        assert_eq!(pp(DAY / 2), "12 hours");
        assert_eq!(pp(DAY), "1 day");
        assert_eq!(pp(10 * DAY), "10 days");

        assert_eq!(pp(DAY + MINUTE - 1), "1 day");
        assert_eq!(pp(DAY + MINUTE), "1 day 1 minute");
        assert_eq!(pp(DAY - 1), "23 hours 59 minutes");

        assert_eq!(pp(2 * DAY - 1), "1 day 23 hours 59 minutes");
    }
}
