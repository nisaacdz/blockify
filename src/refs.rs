use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use chrono::{Datelike, Timelike};

use crate::io::UnitBase;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Range {
    begin: u64,
    end: u64,
}

impl Range {
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

pub trait Detail {
    type Item;

    fn title(&self) -> &str;

    fn item(&self) -> &Self::Item;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixedDetails {
    Bool(detailsamples::BoolDetail),
    Int(detailsamples::IntDetail),
    Float(detailsamples::FloatDetail),
    Text(detailsamples::TextDetail),
}


pub mod detailsamples {
    use super::Deserialize;
    use super::Detail;
    use super::Serialize;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IntDetail {
        title: String,
        item: usize,
    }

    impl Detail for IntDetail {
        type Item = usize;

        fn title(&self) -> &str {
            &self.title
        }

        fn item(&self) -> &Self::Item {
            &self.item
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BoolDetail {
        title: String,
        item: bool,
    }

    impl Detail for BoolDetail {
        type Item = bool;

        fn title(&self) -> &str {
            &self.title
        }

        fn item(&self) -> &Self::Item {
            &self.item
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FloatDetail {
        title: String,
        item: f64,
    }

    impl Detail for FloatDetail {
        type Item = f64;

        fn title(&self) -> &str {
            &self.title
        }

        fn item(&self) -> &Self::Item {
            &self.item
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextDetail {
        title: String,
        item: String,
    }

    impl Detail for TextDetail {
        type Item = String;

        fn title(&self) -> &str {
            &self.title
        }

        fn item(&self) -> &Self::Item {
            &self.item
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    details: Vec<FixedDetails>,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }
    pub fn get<T>(&self, title: &str) -> &T {
        todo!()
    }

    pub fn details(&self) -> &Vec<FixedDetails> {
        &self.details
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenID {
    value: String,
}

impl GenID {
    pub fn generate() -> Self {
        Self {
            value: crate::sec::quick_id(),
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
