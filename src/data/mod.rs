use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[cfg(feature = "unit")]
mod unit;

#[cfg(feature = "unit")]
pub use unit::*;

#[derive(Debug, Clone)]
pub struct Image {
    img: DynamicImage,
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        Image { img }
    }
}

impl Eq for Image {}

impl std::hash::Hash for Image {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.img.as_bytes())
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.img.as_bytes() == other.img.as_bytes()
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D: serde::Deserializer<'de>>(dz: D) -> Result<Self, D::Error> {
        let buffer = <&[u8]>::deserialize(dz)?;
        let img = image::load_from_memory(buffer).map_err(serde::de::Error::custom)?;
        Ok(Image::from(img))
    }
}

impl Serialize for Image {
    fn serialize<S: serde::Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        sz.serialize_bytes(self.img.as_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Detail {
    Text(String),
    Integer(isize),
    Bytes(Box<[u8]>),
    TimeStamp(TimeStamp),
    Image(Image),
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

impl Default for TimeStamp {
    fn default() -> Self {
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
    fn to_timestamp(&self) -> TimeStamp;
}

impl<T: chrono::TimeZone> ToTimeStamp for chrono::DateTime<T> {
    fn to_timestamp(&self) -> TimeStamp {
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

impl<F: ToTimeStamp> From<F> for TimeStamp {
    fn from(value: F) -> Self {
        value.to_timestamp()
    }
}

impl ToTimeStamp for chrono::NaiveDateTime {
    fn to_timestamp(&self) -> TimeStamp {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
