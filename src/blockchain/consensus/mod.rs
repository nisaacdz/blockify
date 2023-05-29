use crate::{chain::Chain, record::Record};

pub mod puzzles;

pub trait ConsensusProtocol {
    type RecordType: Record; // transaction type
    fn validate<B: Chain>(&self, block: B) -> bool;
}
