mod nodestuff;

pub use nodestuff::*;

use crate::{PublicKey, data::Units, record::{SignedRecord, Record}, block::{Block, UnchainedInstance}, chain::Chain};

pub trait Node {
    type RecordType: Record;
    type BlockType: Block<RecordType = Self::RecordType>;
    type ChainType: Chain<RecordType = Self::BlockType, BlockType = Self::BlockType>;
    fn publish(&mut self, record: SignedRecord<Self::RecordType>) -> Result<Feedback, NodeError>;
    fn chain(&self) -> Self::ChainType;
    fn broadcast(&self, block: Self::BlockType) -> Result<Feedback, NodeError>;
    fn append(&self, block: UnchainedInstance<Self::RecordType>, proof: impl MinerProof) -> Result<Feedback, NodeError>;
    fn broadcast_pool()
}

pub trait MinerProof {
    fn verify(&self) -> bool;
}


pub enum Feedback {}

pub trait Peer {
    const UNIT_SIZE: usize;
    fn public_key(&self) -> &PublicKey;
    #[cfg(feature = "unit")]
    fn units(&self) -> Units<{ Self::UNIT_SIZE }>;
}

pub trait Miner {
    fn verify(&self) -> bool;
}
