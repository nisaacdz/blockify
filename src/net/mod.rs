pub mod node;

pub trait Peer {
    fn public_key(&self) -> &[u8];
    fn units(&self) -> &crate::axs::unit::Units;
}

pub trait Miner {
    fn verify(&self) -> bool;
}