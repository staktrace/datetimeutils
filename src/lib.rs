use std::fmt;
use std::time::{Duration, SystemTime, SystemTimeError};

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

pub fn day_abbrev_string(day: Day) -> &'static str {
    &day_string(day)[0..3]
}

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

pub fn month_abbrev_string(month: Month) -> &'static str {
    &month_string(month)[0..3]
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", month_string(*self))
    }
}

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

pub fn seconds_in_day() -> u64 {
    24 * 60 * 60
}

pub fn seconds_in_hour() -> u64 {
    60 * 60
}

pub fn seconds_in_minute() -> u64 {
    60
}

pub struct PostEpochTime {
    delta: Duration,
}

impl PostEpochTime {
    pub fn from(st: &SystemTime) -> Result<Self, SystemTimeError> {
        Ok(PostEpochTime {
            delta: st.duration_since(SystemTime::UNIX_EPOCH)?
        })
    }

    pub fn days_since_epoch(&self) -> u64 {
        self.delta.as_secs() / seconds_in_day()
    }

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

    pub fn year(&self) -> u64 {
        self.year_split().0
    }

    pub fn day_of_year(&self) -> u64 {
        self.year_split().1
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

    pub fn month(&self) -> Month {
        self.month_split().0
    }

    pub fn day_of_month(&self) -> u64 {
        self.month_split().1 + 1
    }

    pub fn second_in_day(&self) -> u64 {
        self.delta.as_secs() % seconds_in_day()
    }

    pub fn hour(&self) -> u64 {
        self.second_in_day() / seconds_in_hour()
    }

    pub fn second_in_hour(&self) -> u64 {
        self.second_in_day() % seconds_in_hour()
    }

    pub fn minute(&self) -> u64 {
        self.second_in_hour() / seconds_in_minute()
    }

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
