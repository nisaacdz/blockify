use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Detail {
    Text(String),
    Int(usize),
    Bytes(Box<[u8]>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetaData {
    details: Vec<Detail>,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::new()
    }

    pub fn get<T>(&self, _title: &str) -> &T {
        todo!()
    }

    pub fn details(&self) -> &Vec<Detail> {
        &self.details
    }
}
