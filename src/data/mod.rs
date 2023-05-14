use std::ops::Sub;

use serde::{Deserialize, Serialize};

pub trait UnitManager {
    fn all_units(&self);
    fn all_units_raw(&self);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Micron {
    val: u64,
    cat: ID,
}

impl Micron {
    pub fn create(val: u64, cat: ID) -> Self {
        Self { val, cat }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Units {
    vals: Vec<Micron>,
}

impl Units {
    pub fn new(microns: Vec<Micron>) -> Self {
        Self { vals: microns }
    }
    pub fn get_value(&self) -> f64 {
        todo!()
    }

    pub fn microns(&self) -> &Vec<Micron> {
        &self.vals
    }
}

impl Sub<Micron> for Units {
    type Output = Self;

    fn sub(mut self, rhs: Micron) -> Self::Output {
        for m in self.vals.iter_mut() {
            if m.cat == rhs.cat {
                m.val -= rhs.val;
            }
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    img: image::DynamicImage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Detail {
    Text(String),
    Integer(isize),
    Bytes(Box<[u8]>),
    TimeStamp(TimeStamp),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetaData {
    details: Box<[Detail]>,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            details: Vec::with_capacity(0).into_boxed_slice(),
        }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::new()
    }

    pub fn get<T>(&self, _title: &str) -> &T {
        todo!()
    }

    pub fn details(&self) -> &[Detail] {
        &self.details
    }
}

use chrono::{Datelike, Timelike};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        Self {
            millis: 0,
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        }
    }
}
pub trait ToTimeStamp {
    fn to_local_timestamp(&self) -> TimeStamp;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockRange {
    begin: u64,
    end: u64,
}

impl BlockRange {
    pub fn new(begin: u64, end: u64) -> Self {
        Self { begin, end }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ID {
    value: String,
}

impl ID {
    pub fn random() -> Self {
        Self {
            value: crate::crypto::quick_id(10),
        }
    }
}
