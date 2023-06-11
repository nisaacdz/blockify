//! This module contains different data types that of some importance to this crate.
//!

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike};
use serde::{Deserialize, Serialize};

mod unit;

pub use unit::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Detail {
    Text(String),
    Integer(i64),
    Bytes(Box<[u8]>),
    Timestamp(Timestamp),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metadata {
    details: Vec<Detail>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            details: Vec::with_capacity(0),
        }
    }

    pub fn push(&mut self, value: Detail) {
        self.details.push(value)
    }

    pub fn pop(&mut self) -> Option<Detail> {
        self.details.pop()
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::new()
    }

    pub fn details(&self) -> &[Detail] {
        &self.details
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timestamp {
    secs: u64,
}

pub trait ToTimestamp {
    fn to_timestamp(&self) -> Timestamp;
}

impl ToTimestamp for u64 {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp::from_secs(*self)
    }
}

#[test]
fn test_timestamp_for_u64() {
    let val = 33u64;
    let _timestamp = val.to_timestamp();
    assert!(true, "incomplete implementation of to_timestamp for u64")
}

impl<T: chrono::TimeZone> ToTimestamp for chrono::DateTime<T> {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp {
            secs: self.timestamp() as _,
        }
    }
}

impl<F: ToTimestamp> From<F> for Timestamp {
    fn from(value: F) -> Self {
        value.to_timestamp()
    }
}

impl ToTimestamp for chrono::NaiveDateTime {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp {
            secs: self.timestamp() as _,
        }
    }
}

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

impl Into<u8> for Month {
    fn into(self) -> u8 {
        match self {
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
}

impl From<u8> for Month {
    fn from(value: u8) -> Month {
        match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => unreachable!(),
        }
    }
}

impl Timestamp {
    pub fn date_time<Z: TimeZone>(self, tz: &Z) -> DateTime<Z> {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let res = tz.from_utc_datetime(&utc);
        res
    }

    pub fn year(self) -> u16 {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        dt.year() as _
    }
    pub fn month(self) -> Month {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        (dt.month() as u8).into()
    }
    pub fn day(self) -> u8 {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        dt.day() as _
    }
    pub fn hour(self) -> u8 {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        dt.hour() as _
    }
    pub fn minute(self) -> u8 {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        dt.minute() as _
    }
    pub fn second(self) -> u8 {
        let utc = NaiveDateTime::from_timestamp_opt(self.secs as _, 0).unwrap();
        let dt = utc.and_utc();
        dt.second() as _
    }

    pub fn from_secs(secs: u64) -> Self {
        Self { secs }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BufID {
    value: [u8; 16],
}

impl BufID {
    pub fn random() -> Self {
        Self::new(crate::random_bytes())
    }

    pub fn new(value: [u8; 16]) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        hex::encode(&self.value)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Nonce {
    pub nonce: u64,
}

impl Nonce {
    pub fn new(nonce: u64) -> Self {
        Nonce { nonce }
    }
}

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Nonce::new(value)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pub pos: u64,
}

impl Position {
    pub fn new(pos: u64) -> Self {
        Position { pos }
    }

    pub fn pos(&self) -> u64 {
        self.pos
    }
}

impl From<u64> for Position {
    fn from(value: u64) -> Self {
        Position::new(value)
    }
}
