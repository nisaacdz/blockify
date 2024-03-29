use serde::{Deserialize, Serialize};

pub trait UnitManager {
    fn all_units(&self);
    fn all_units_raw(&self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Quantity {
    val: i32,
}

impl From<i32> for Quantity {
    fn from(val: i32) -> Self {
        Self { val }
    }
}

impl Quantity {
    pub fn none() -> Self {
        Self::new(0)
    }
    pub fn new(val: i32) -> Self {
        Self { val }
    }
    /// increases the internal count by 1 and returns the new count
    pub fn increment(&mut self) -> i32 {
        self.val += 1;
        self.val
    }
    /// increases the internal count by `val` and returns the new count
    pub fn increment_by(&mut self, val: i32) -> i32 {
        self.val += val;
        self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Micron {
    id: i32,
}

impl From<i32> for Micron {
    fn from(id: i32) -> Self {
        Self::new(id)
    }
}

impl Micron {
    pub fn new(id: i32) -> Self {
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

impl MicQuan {
    #[cfg(debug_assertions)]
    pub fn debug_with(m: i32, q: i32) -> Self {
        Self::new(m.into(), q.into())
    }

    pub fn new(micron: Micron, quantity: Quantity) -> Self {
        Self { micron, quantity }
    }

    pub fn increment(&mut self) -> Quantity {
        self.quantity.increment().into()
    }
}

impl<const N: usize> Serialize for Units<N> {
    fn serialize<S: serde::Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        self.value.into_iter().collect::<Vec<_>>().serialize(sz)
    }
}

impl<'d, const N: usize> Deserialize<'d> for Units<N> {
    fn deserialize<D: serde::Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let vec = <Vec<MicQuan>>::deserialize(dz)?;
        let mut real = [MicQuan::default(); N];

        if vec.len() != N {
            return Err(serde::de::Error::custom(format!(
                "Expected array of length {}, found length {}",
                N,
                vec.len()
            )));
        }

        for i in 0..N {
            real[i] = vec[i];
        }

        Ok(real.into())
    }
}

#[cfg(debug_assertions)]
mod test_units {
    #[allow(unused)]
    use super::{MicQuan, Units};

    #[test]
    fn test_serde() {
        let units = Units::new([
            MicQuan::debug_with(0, 0),
            MicQuan::debug_with(1, 1),
            MicQuan::debug_with(2, 2),
        ]);
        let serde_str = serde_json::to_string(&units).expect("couldn't stringify units");

        let gen_units =
            serde_json::from_str::<Units<3>>(&serde_str).expect("couldn't unstringify units");

        assert_eq!(units, gen_units);
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
