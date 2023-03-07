use std::{
    fmt::Debug,
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex},
};

use chrono::{Datelike, Timelike};
use io::UnitBase;
use serde::{Deserialize, Serialize};

pub mod errs;
pub mod io;
pub mod net;
pub mod sec;
pub mod tests;
pub mod trans;
pub mod ver;

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct GenID {
    value: String,
}

impl Debug for GenID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl Display for GenID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl GenID {
    pub fn generate() -> Self {
        Self {
            value: sec::quick_id(),
        }
    }
}

pub struct UnitManager {
    _db: Arc<Mutex<dyn UnitBase>>,
}

impl UnitManager {
    pub fn all_units(&self) -> f64 {
        todo!()
    }

    pub fn all_units_raw(&self) -> u128 {
        todo!()
    }
}

pub struct Unit {
    val: u64,
    cat: GenID,
}

impl Unit {
    pub fn get_value(&self) -> f64 {
        todo!()
    }

    pub fn get_value_raw(&self) -> u64 {
        self.val
    }

    pub fn id(&self) -> GenID {
        self.cat.clone()
    }
}

pub trait Detail {
    fn title(&self) -> String;
    fn item(&self) -> Rc<dyn std::any::Any>;
}

#[derive(Clone)]
pub struct MetaData {
    details: Vec<Rc<dyn Detail>>,
}

impl MetaData {
    pub fn new() -> Self {
        Self { details: vec![] }
    }
    pub fn get(&self, title: &str) -> Option<Rc<dyn Detail>> {
        for detail in &self.details {
            if detail.title() == title {
                return Some(detail.clone());
            }
        }
        None
    }

    pub fn details(&self) -> &Vec<Rc<dyn Detail>> {
        &self.details
    }
}
