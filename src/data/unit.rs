use serde::{Deserialize, Serialize};

use super::ID;
use std::ops::Sub;

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