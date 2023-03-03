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

impl TimeStamp {
    pub fn debug() -> Self {
        Self { millis: todo!(), second: todo!(), minute: todo!(), hour: todo!(), day: todo!(), month: todo!(), year: todo!()  }
    }
}

pub trait ToTimeStamp {
    fn to_local_timestamp(&self) -> TimeStamp;
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

impl ToTimeStamp for chrono::NaiveDateTime {
    fn to_local_timestamp(&self) -> TimeStamp {
        TimeStamp {
            millis: 0,
            second: self.second() as u8,
            minute: self.minute() as u8,
            hour: self.hour() as u8,
            day: self.day() as u8,
            month: self.month() as u8,
            year: self.year() as u16,
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
