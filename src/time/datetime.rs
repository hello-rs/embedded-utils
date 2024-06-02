use crate::time::TimeZone;

/// DateTime is a Unix and UTC conversion util.
///
/// Example of converting a Unix to a DateTime UTC.
/// ```rust,no_run
/// use embedded_utils::time::{DateTime, TimeZone};
///
/// let datetime = DateTime::from_unix_millis(1704067199998, TimeZone::UTC);
/// println!("datetime: {:?}", datetime);
/// ```
#[derive(core::fmt::Debug)]
pub struct DateTime {
    /// Year
    pub year: u32,
    /// Month
    pub month: u8,
    /// Day
    pub day: u8,
    /// Hour
    pub hour: u8,
    /// Minute
    pub minute: u8,
    /// Second
    pub second: u8,
    /// Millisecond
    pub millisecond: u16,
    /// Timezone
    pub timezone: TimeZone,
}

const MILLISECONDS_PER_SECOND: u64 = 1000;
const SECONDS_PER_MINUTE: u64 = 60;
const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const DAYS_PER_YEAR: u64 = 365;
const DAYS_PER_LEAP_YEAR: u64 = 366;

const MONTHS: [[u8; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Normal year
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Leap year
];

impl DateTime {
    /// Create a new DateTime from a Unix timestamp in seconds.
    #[inline]
    pub fn from_unix_secs(timestamp: u64, timezone: TimeZone) -> DateTime {
        return Self::from_unix_millis(timestamp * 1000, timezone);
    }

    /// Create a new DateTime from a Unix timestamp in milliseconds.
    pub fn from_unix_millis(timestamp: u64, timezone: TimeZone) -> DateTime {
        let mut milliseconds = timestamp;
        let mut seconds = milliseconds / MILLISECONDS_PER_SECOND;
        let mut minutes = (seconds / SECONDS_PER_MINUTE) as i32;
        let mut hours = minutes / MINUTES_PER_HOUR;
        let mut days = hours / HOURS_PER_DAY;

        milliseconds %= MILLISECONDS_PER_SECOND;
        seconds %= SECONDS_PER_MINUTE;
        minutes %= MINUTES_PER_HOUR;
        hours %= HOURS_PER_DAY;
        hours += timezone.get_offset() / 3600;
        if hours < 0 {
            hours += HOURS_PER_DAY as i32;
            days -= 1;
        } else if hours >= HOURS_PER_DAY as i32 {
            hours -= HOURS_PER_DAY as i32;
            days += 1;
        }

        let mut years = 1970;
        while days
            >= if is_leap_year(years) {
                DAYS_PER_LEAP_YEAR
            } else {
                DAYS_PER_YEAR
            } as i32
        {
            days -= if is_leap_year(years) {
                DAYS_PER_LEAP_YEAR
            } else {
                DAYS_PER_YEAR
            } as i32;
            years += 1;
        }

        let mut month = 0;
        while days >= MONTHS[is_leap_year(years) as usize][month as usize] as i32 {
            days -= MONTHS[is_leap_year(years) as usize][month as usize] as i32;
            month += 1;
        }

        DateTime {
            year: years,
            month: month + 1,
            day: days as u8 + 1,
            hour: hours as u8,
            minute: minutes as u8,
            second: (seconds % SECONDS_PER_MINUTE) as u8,
            millisecond: milliseconds as u16,
            timezone,
        }
    }
}

/// Check if a year is a leap year.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use crate::time::{DateTime, TimeZone};

    #[test]
    fn from_unix_secs_works() {
        // converting a Unix to a DateTime UTC.
        let datetime = DateTime::from_unix_secs(1704067199, TimeZone::UTC);
        assert_eq!(datetime.year, 2023, "year: {}", datetime.year);
        assert_eq!(datetime.month, 12, "month: {}", datetime.month);
        assert_eq!(datetime.day, 31, "day: {}", datetime.day);
        assert_eq!(datetime.hour, 23, "hour: {}", datetime.hour);
        assert_eq!(datetime.minute, 59, "minute: {}", datetime.minute);
        assert_eq!(datetime.second, 59, "second: {}", datetime.second);
        assert_eq!(
            datetime.millisecond, 0,
            "millisecond: {}",
            datetime.millisecond
        );
        assert_eq!(
            datetime.timezone.get_offset(),
            TimeZone::UTC.get_offset(),
            "timezone: {:?}",
            datetime.timezone.get_offset()
        );
    }

    #[test]
    fn from_unix_millis_works() {
        // converting a Unix to a DateTime AsiaShanghai.
        let datetime = DateTime::from_unix_millis(1704067199998, TimeZone::AsiaShanghai);
        assert_eq!(datetime.year, 2024, "year: {}", datetime.year);
        assert_eq!(datetime.month, 1, "month: {}", datetime.month);
        assert_eq!(datetime.day, 1, "day: {}", datetime.day);
        assert_eq!(datetime.hour, 7, "hour: {}", datetime.hour);
        assert_eq!(datetime.minute, 59, "minute: {}", datetime.minute);
        assert_eq!(datetime.second, 59, "second: {}", datetime.second);
        assert_eq!(
            datetime.millisecond, 998,
            "millisecond: {}",
            datetime.millisecond
        );
        assert_eq!(
            datetime.timezone.get_offset(),
            TimeZone::AsiaShanghai.get_offset(),
            "timezone: {:?}",
            datetime.timezone.get_offset()
        );
    }
}
