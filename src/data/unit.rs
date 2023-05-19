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
    value: [MicQuan; N],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MicQuan {
    micron: Micron,
    quantity: Quantity,
}

impl<const N: usize> Serialize for Units<N> {
    fn serialize<S: serde::Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        self.value.into_iter().collect::<Vec<_>>().serialize(sz)
    }
}

impl<'d, const N: usize> Deserialize<'d> for Units<N> {
    fn deserialize<D: serde::Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let val = <Vec<(Micron, Quantity)>>::deserialize(dz)?;

        let mut real = [(Micron::default(), Quantity::default()); N];

        for i in 0..N {
            real[i] = val[i];
        }

        Ok(real.into())
    }
    
}

impl<const N: usize> From<[MicQuan; N]> for Units<N> {
    fn from(value: [MicQuan; N]) -> Self {
        Units::new(value)
    }
}

impl<const N: usize> Units<N> {
    pub fn new(value: [MicQuan; N]) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> f64 {
        todo!()
    }
}
