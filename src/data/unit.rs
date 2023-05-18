use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub trait UnitManager {
    fn all_units(&self);
    fn all_units_raw(&self);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Category {
    cat: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Micron {
    val: u64,
    cat: Category,
}

impl Micron {
    pub fn new(val: u64, cat: Category) -> Self {
        Self { val, cat }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Units {
    vals: HashMap<Micron, u64>,
}

impl Units {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }
    pub fn get_value(&self) -> f64 {
        todo!()
    }
}
