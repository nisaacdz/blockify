use serde::{Deserialize, Serialize};

pub trait UnitManager {
    fn all_units(&self);
    fn all_units_raw(&self);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Category {
    cat: i32,
}

impl Category {
    pub fn new(cat: i32) -> Self {
        Self { cat }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Quantity {
    val: i32,
}

impl Quantity {
    pub fn none() -> Self {
        Self { val: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Micron {
    id: Category,
}

impl Micron {
    pub fn new(id: Category) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Units<const N: usize> {
    value: [(Micron, Quantity); N],
}

impl<const N: usize> From<[(Micron, Quantity); N]> for Units<N> {
    fn from(value: [(Micron, Quantity); N]) -> Self {
        Units { value }
    }
}

impl<const N: usize> Units<N> {
    pub fn new(value: [(Micron, Quantity); N]) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> f64 {
        todo!()
    }
}
