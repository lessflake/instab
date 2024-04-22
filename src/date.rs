// use std::time::SystemTime;

const FEB_28: u16 = 58;
const DEC_31: u16 = 365;
const UNIX_START_YEAR: u16 = 1970;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Date {
    year: u16,
    day: u16,
}

#[cfg(windows)]
fn unix_timestamp() -> u64 {
    use winapi::shared::minwindef::FILETIME;
    use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;
    let mut ft = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    unsafe {
        GetSystemTimePreciseAsFileTime(&mut ft as *mut _);
    }
    (ft.dwLowDateTime as u64 + ((ft.dwHighDateTime as u64) << 32)) / 10000000 - 11644473600
}

#[cfg(not(windows))]
fn unix_timestamp() -> u64 {
    let mut unix = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut unix as *mut _) };
    unix.tv_sec as u64
}

impl Date {
    pub fn today() -> Self {
        let mut t = unix_timestamp();
        let mut year = UNIX_START_YEAR;

        loop {
            let length = year_length(year);
            if t < length {
                break;
            }
            t -= length;
            year += 1;
        }

        let day = (t / (60 * 60 * 24)) as u16;
        let day = normalize_leap_year(year, day);

        Self { year, day }
    }

    pub const fn from_ymd(year: u16, month: u8, day: u8) -> Self {
        let mut day = day as u16 - 1;
        let month = month - 1;
        let mut inc_month = 0;
        loop {
            if month == inc_month {
                break;
            }
            day += days_per_month(year, inc_month) as u16;
            inc_month += 1;
        }

        let day = normalize_leap_year(year, day);

        Self { year, day }
    }

    pub const fn ymd(&self) -> (u16, u8, u8) {
        let (month, day) = self.month_day();
        (self.year, month, day)
    }

    pub const fn month_day(&self) -> (u8, u8) {
        let mut month = 0;
        let mut day = self.day;

        loop {
            let length = days_per_month(2020, month);
            if day < length as u16 {
                break;
            }
            day -= length as u16;
            month += 1;
        }

        (month + 1, day as u8 + 1)
    }

    pub const fn day(&self) -> u16 {
        self.day
    }

    pub const fn year(&self) -> u16 {
        self.year
    }

    pub const fn successor(&self) -> Self {
        let year = self.year;
        let day = self.day;

        if day == DEC_31 {
            Self {
                year: year + 1,
                day: 0,
            }
        } else if day == FEB_28 && !is_leap_year(year) {
            Self { year, day: day + 2 }
        } else {
            Self { year, day: day + 1 }
        }
    }
}

impl core::fmt::Display for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (year, month, day) = self.ymd();
        write!(f, "{}-{:02}-{:02}", year, month, day)
    }
}

pub fn future_days() -> impl Iterator<Item = Date> {
    let mut date = Date::today();
    core::iter::from_fn(move || {
        let ret = date;
        date = date.successor();
        Some(ret)
    })
}

// impl Year {
//     const fn leap(&self) -> bool {
//         (self.0 % 4 == 0) && (self.0 % 100 != 0 || self.0 % 400 == 0)
//     }
// }

const fn normalize_leap_year(year: u16, mut day: u16) -> u16 {
    if !is_leap_year(year) && day > FEB_28 {
        day += 1;
    }
    day
}

const fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

const fn year_length(year: u16) -> u64 {
    if !is_leap_year(year) {
        60 * 60 * 24 * 365
    } else {
        60 * 60 * 24 * 366
    }
}

const fn days_per_month(year: u16, month: u8) -> u8 {
    match month {
        0 => 31,
        1 => {
            if !is_leap_year(year) {
                28
            } else {
                29
            }
        }
        2 => 31,
        3 => 30,
        4 => 31,
        5 => 30,
        6 => 31,
        7 => 31,
        8 => 30,
        9 => 31,
        10 => 30,
        11 => 31,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successor_is_successor() {
        let date = Date::from_ymd(2021, 2, 28);
        let succ = date.successor();
        assert_eq!(succ.day, 60);

        let date = Date::from_ymd(2020, 2, 28);
        let succ = date.successor();
        assert_eq!(succ.day, 59);

        let succ = succ.successor();
        assert_eq!(succ.day, 60);

        let date = Date::from_ymd(2020, 12, 31);
        let succ = date.successor();
        assert_eq!(succ.day, 0);

        let date = Date::from_ymd(2021, 12, 31);
        let succ = date.successor();
        assert_eq!(succ.day, 0);
    }

    #[test]
    fn months_and_days_match() {
        let date = Date::from_ymd(2020, 2, 28);
        let (month, day) = date.month_day();
        assert_eq!(month, 2);
        assert_eq!(day, 28);

        let (month, day) = date.successor().month_day();
        assert_eq!(month, 2);
        assert_eq!(day, 29);

        let date = Date::from_ymd(2021, 2, 28);
        let (month, day) = date.month_day();
        assert_eq!(month, 2);
        assert_eq!(day, 28);

        let (month, day) = date.successor().month_day();
        assert_eq!(month, 3);
        assert_eq!(day, 1);
    }

    #[test]
    fn time() {
        use libc_print::std_name::println;
        let unix_timestamp = unix_timestamp();
        let mut t = unix_timestamp;
        let mut year = 1970;

        t += year_length(year + 1);

        loop {
            let length = year_length(year);
            if t < length {
                break;
            }
            t -= length;
            year += 1;
        }

        let day = t % (60 * 60 * 24);
        let hours = day / (60 * 60);
        let mins = (day % (60 * 60)) / 60;
        let secs = day % 60;

        let day_of_year = t / (60 * 60 * 24);
        let mut month = 0;
        let mut month_day = day_of_year as u16;

        loop {
            let length = days_per_month(year, month);
            if month_day < length as u16 {
                break;
            }
            month_day -= length as u16;
            month += 1;
        }

        let mut day_of_year_including_feb29 = day_of_year;
        if !is_leap_year(year) {
            if month > 1 {
                day_of_year_including_feb29 += 1;
            }
        }

        println!("{}", t);
        println!("it is the year {}", year);
        println!("it is the time {}:{}:{}", hours, mins, secs);
        println!("it is day {} of month {}", month_day, month);
        println!("it is day of year {}", day_of_year);
        println!("ordinal366 is {}", day_of_year_including_feb29);
    }
}
