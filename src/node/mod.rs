mod nodestuff;

pub use nodestuff::*;

use crate::PublicKey;

pub trait Peer {
    // const UnitSize: usize;
    fn public_key(&self) -> &PublicKey;
    //#[cfg(feature = "unit")]
    //fn units(&self) -> &Units<{ Self::UnitSize }>;
}

pub trait Miner {
    fn verify(&self) -> bool;
}
