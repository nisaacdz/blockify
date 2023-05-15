#[cfg(feature = "blockchain")]
mod nodestuff;

#[cfg(feature = "blockchain")]
pub use nodestuff::*;

#[cfg(feature = "unit")]
pub trait Peer {
    fn public_key(&self) -> &[u8];
    fn units(&self) -> &crate::data::Units;
}

pub trait Miner {
    fn verify(&self) -> bool;
}
