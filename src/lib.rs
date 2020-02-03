use std::fmt;
use std::time::{Duration, SystemTime, SystemTimeError};

/// Enum with the seven days of the week.
#[derive(Debug, Clone, Copy)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// Maps the `Day` enum to a string representation, e.g. "Monday".
pub fn day_string(day: Day) -> &'static str {
    match day {
        Day::Sunday => "Sunday",
        Day::Monday => "Monday",
        Day::Tuesday => "Tuesday",
        Day::Wednesday => "Wednesday",
        Day::Thursday => "Thursday",
        Day::Friday => "Friday",
        Day::Saturday => "Saturday",
    }
}

/// Maps the `Day` enum to a shortened string representation, e.g. "Mon".
pub fn day_abbrev_string(day: Day) -> &'static str {
    &day_string(day)[0..3]
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", day_string(*self))
    }
}

/// Enum with the months of the year.
#[derive(Debug, Clone, Copy)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// Maps the `Month` enum to a string representation, e.g. "January".
pub fn month_string(month: Month) -> &'static str {
    match month {
        Month::January => "January",
        Month::February => "February",
        Month::March => "March",
        Month::April => "April",
        Month::May => "May",
        Month::June => "June",
        Month::July => "July",
        Month::August => "August",
        Month::September => "September",
        Month::October => "October",
        Month::November => "November",
        Month::December => "December",
    }
}

/// Maps the `Month` enum to a shortened string representation, e.g. "Jan".
pub fn month_abbrev_string(month: Month) -> &'static str {
    &month_string(month)[0..3]
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", month_string(*self))
    }
}

/// Takes in a year (e.g. 2019) and returns the number of days in that year.
pub fn days_in_year(year: u64) -> u64 {
    if year % 400 == 0 {
        366
    } else if year % 100 == 0 {
        365
    } else if year % 4 == 0 {
        366
    } else {
        365
    }
}

/// Takes in a year and month (e.g. 2020, February) and returns the number of days in that month.
pub fn days_in_month(year: u64, month: Month) -> u64 {
    match month {
        Month::January => 31,
        Month::February if days_in_year(year) == 366 => 29,
        Month::February => 28,
        Month::March => 31,
        Month::April => 30,
        Month::May => 31,
        Month::June => 30,
        Month::July => 31,
        Month::August => 31,
        Month::September => 30,
        Month::October => 31,
        Month::November => 30,
        Month::December => 31,
    }
}

/// Converts a `Month` enum to an integer in the range 1-12.
pub fn index_from_month(month: Month) -> u64 {
    match month {
        Month::January => 1,
        Month::February => 2,
        Month::March => 3,
        Month::April => 4,
        Month::May => 5,
        Month::June => 6,
        Month::July => 7,
        Month::August => 8,
        Month::September => 9,
        Month::October => 10,
        Month::November => 11,
        Month::December => 12,
    }
}

/// Converts an integer in the range 1-12 into the corresponding `Month` enum.
/// Values outside the 1-12 range are converted to `None`.
pub fn month_from_index(index: u64) -> Option<Month> {
    match index {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}

/// Returns the number of seconds in a day.
pub fn seconds_in_day() -> u64 {
    24 * 60 * 60
}

/// Returns the number of seconds in an hour.
pub fn seconds_in_hour() -> u64 {
    60 * 60
}

/// Returns the number of seconds in a minute.
pub fn seconds_in_minute() -> u64 {
    60
}

/// Conceptually this is a thin wrapper for `std::time::SystemTime`, but provides
/// more useful functions. The impl of this struct has functions that allow easily
/// extracting the year/month/date/etc. for the given point in time. In actual fact
/// the internal representation of this struct is a `Duration` since the unix epoch,
/// so that error-handling is only required once upon creating the instance, and
/// not for each attempt at extracting date/time fields.
pub struct PostEpochTime {
    delta: Duration,
}

impl PostEpochTime {
    /// Create a `PostEpochTime` from a `SystemTime`. The `SystemTime` must be temporally
    /// in the future relative to the unix epoch, or an error will be returned.
    pub fn from(st: &SystemTime) -> Result<Self, SystemTimeError> {
        Ok(PostEpochTime {
            delta: st.duration_since(SystemTime::UNIX_EPOCH)?
        })
    }

    /// Create a `PostEpochTime` for the current instant. The current instant must be
    /// in the future relative to the unix epoch, or an error will be returned.
    pub fn now() -> Result<Self, SystemTimeError> {
        Self::from(&SystemTime::now())
    }

    /// Returns the number of milliseconds passed since the unix epoch.
    pub fn milliseconds_since_epoch(&self) -> u128 {
        self.delta.as_millis()
    }

    /// Returns the number of microseconds passed since the unix epoch.
    pub fn microseconds_since_epoch(&self) -> u128 {
        self.delta.as_micros()
    }

    /// Returns the number of nanoseconds passed since the unix epoch.
    pub fn nanoseconds_since_epoch(&self) -> u128 {
        self.delta.as_nanos()
    }

    /// Returns the number of complete seconds passed since the unix epoch.
    pub fn seconds_since_epoch(&self) -> u64 {
        self.delta.as_secs()
    }

    /// Returns the number of complete days passed since the unix epoch.
    pub fn days_since_epoch(&self) -> u64 {
        self.delta.as_secs() / seconds_in_day()
    }

    /// Returns the day of the week that this point in time falls on.
    pub fn day_of_week(&self) -> Day {
        match self.days_since_epoch() % 7 {
            0 => Day::Thursday,
            1 => Day::Friday,
            2 => Day::Saturday,
            3 => Day::Sunday,
            4 => Day::Monday,
            5 => Day::Tuesday,
            6 => Day::Wednesday,
            _ => panic!("Modulo operator is broken"),
        }
    }

    fn year_split(&self) -> (u64, u64) {
        let mut days = self.days_since_epoch();
        let mut year = 1970;
        loop {
            let in_year = days_in_year(year);
            if days < in_year {
                break;
            }
            days -= in_year;
            year += 1;
        }
        (year, days)
    }

    /// Returns the year (e.g. 2020) this point in time falls on.
    pub fn year(&self) -> u64 {
        self.year_split().0
    }

    /// Returns the day of the year for this point in time (1-indexed).
    /// A return value of 1 indicates January 1, a value of 2 indicates January 2,
    /// and so on. If the year is a leap year the largest returned value
    /// would be 366, and for non-leap years it would be 365.
    pub fn day_of_year(&self) -> u64 {
        self.year_split().1 + 1
    }

    fn month_split(&self) -> (Month, u64) {
        let (year, mut days) = self.year_split();
        let mut month = Month::January;
        loop {
            let in_month = days_in_month(year, month);
            if days < in_month {
                break;
            }
            days -= in_month;
            month = month_from_index(index_from_month(month) + 1).expect("Month should never overflow");
        }
        (month, days)
    }

    /// Returns the month this point in time falls on.
    pub fn month(&self) -> Month {
        self.month_split().0
    }

    /// Returns the day of the month for this point in time (1-indexed).
    /// A return value of 1 means it falls on the first of the month. The maximum
    /// returned value will be 31.
    pub fn day_of_month(&self) -> u64 {
        self.month_split().1 + 1
    }

    /// Returns the second within the day (0-indexed). This will be in the range
    /// 0..86399 (inclusive).
    pub fn second_in_day(&self) -> u64 {
        self.delta.as_secs() % seconds_in_day()
    }

    /// Returns the hour within the day (0-indexed). This will be in the range
    /// 0..23 (inclusive).
    pub fn hour(&self) -> u64 {
        self.second_in_day() / seconds_in_hour()
    }

    /// Returns the second within the hour (0-indexed). This will be in the range
    /// 0..3599 (inclusive).
    pub fn second_in_hour(&self) -> u64 {
        self.second_in_day() % seconds_in_hour()
    }

    /// Returns the minute within the hour (0-indexed). This will be in the range
    /// 0..59 (inclusive).
    pub fn minute(&self) -> u64 {
        self.second_in_hour() / seconds_in_minute()
    }

    /// Returns the second within the minute (0-indexed). This will be in the range
    /// 0..59 (inclusive).
    pub fn second(&self) -> u64 {
        self.delta.as_secs() % seconds_in_minute()
    }
}

impl fmt::Display for PostEpochTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {} {} {} {:02}:{:02}:{:02}", 
            day_abbrev_string(self.day_of_week()),
            self.day_of_month(),
            month_abbrev_string(self.month()),
            self.year(),
            self.hour(),
            self.minute(),
            self.second())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let timestamp = SystemTime::UNIX_EPOCH + Duration::new(1580610340, 123);
        let pet = PostEpochTime::from(&timestamp).unwrap();
        assert_eq!(format!("{}", pet), "Sun, 2 Feb 2020 02:25:40".to_string());
    }
}
