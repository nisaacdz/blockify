use std::fmt::Display;

use chrono::{Datelike, Timelike};

pub mod block;
pub mod errs;
pub mod gen;
pub mod io;
pub mod record;

#[derive(Clone, Copy)]
pub struct TimeStamp {
    millis: u8,
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u16,
}

pub trait ToTimeStamp {
    fn timestamp(&self) -> TimeStamp;
}

#[derive(Clone, Copy)]
pub struct Range {
    begin: u64,
    end: u64,
}

impl Range {
    pub fn new(begin: u64, end: u64) -> Self {
        Self { begin, end }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("({}, {})", self.begin, self.end))
    }
}

impl ToTimeStamp for chrono::DateTime<chrono::Utc> {
    fn timestamp(&self) -> TimeStamp {
        let naive_dt = self.naive_utc();
        TimeStamp {
            millis: 0,
            second: naive_dt.second() as u8,
            minute: naive_dt.minute() as u8,
            hour: naive_dt.hour() as u8,
            day: naive_dt.day() as u8,
            month: naive_dt.month() as u8,
            year: naive_dt.year() as u16,
        }
    }
}

///
///
///
///
///
///
///

impl TimeStamp {
    pub fn year(&self) -> u16 {
        self.year
    }
    pub fn month(&self) -> u8 {
        self.month
    }
    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn hour(&self) -> u8 {
        self.hour
    }
    pub fn minute(&self) -> u8 {
        self.minute
    }
    pub fn second(&self) -> u8 {
        self.second
    }
    pub fn millis(&self) -> u8 {
        self.millis
    }
}
