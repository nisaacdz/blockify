pub mod node;
pub mod nodeserver;

pub trait Peer {
    fn public_key(&self) -> &[u8];
    fn units(&self) -> &crate::data::Units;
}

pub trait Miner {
    fn verify(&self) -> bool;
}
