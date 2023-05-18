use crate::data::Units;

mod nodestuff;

pub use nodestuff::*;

use crate::PublicKey;

pub trait Peer {
    fn public_key(&self) -> &PublicKey;
    #[cfg(feature = "unit")]
    fn units(&self) -> &Units;
}

pub trait Miner {
    fn verify(&self) -> bool;
}
