use std::{
    ops::Sub,
    sync::{Arc, Mutex},
};

use serde::{Serialize, Deserialize};

use crate::refs::ID;

pub struct UnitManager {
    _db: Arc<Mutex<dyn crate::io::UnitBase>>,
}

impl UnitManager {
    pub fn all_units(&self) -> f64 {
        todo!()
    }

    pub fn all_units_raw(&self) -> u128 {
        todo!()
    }
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
