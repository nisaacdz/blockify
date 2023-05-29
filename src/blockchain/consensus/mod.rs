use crate::{block::Block, chain::Chain, record::Record};

pub mod puzzles;

pub trait ConsensusProtocol {
    type RecordType: Record; // transaction type
    type BlockType: Block<RecordType = Self::RecordType>;
    type ChainType: Chain<RecordType = Self::RecordType, BlockType = Self::BlockType>;
    type ConsensusRules: Rules<Self::ChainType>;
    type BranchesType: ChainBranches<Self::ChainType, Self::ConsensusRules>;
    fn validate<B: Chain>(&self, block: B) -> bool;
    fn active_chain(&self) -> Result<Self::ChainType, ConsensusError>;
    fn branches(&self) -> Result<Self::BranchesType, ConsensusError>;
}

pub trait Rules<C: Chain> {
    fn merge(&mut self, branches: Vec<C>) -> Result<C, ConsensusError>;
}

pub trait ChainBranches<C: Chain, R: Rules<C>> {
    fn branches(&self) -> Result<Vec<C>, ConsensusError>;
    /// Merges the branches according to the rules and returns the resulting chain
    fn merge(&mut self, rules: R) -> Result<C, ConsensusError>;
}

pub enum ConsensusError {}
