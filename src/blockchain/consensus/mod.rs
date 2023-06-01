use crate::{
    block::Block,
    chain::Chain,
    error::{DataBaseError, SerdeError},
    record::Record,
};

pub mod puzzles;

pub trait ConsensusProtocol {
    type RecordType: Record; // transaction type
    type BlockType: Block<RecordType = Self::RecordType>;
    type ChainType: Chain<RecordType = Self::RecordType, BlockType = Self::BlockType>;
    type ConsensusRulesType: ConsensusRules<Self::ChainType>;
    type BranchesType: ChainBranches<Self::ChainType, Self::ConsensusRulesType>;
    fn validate<B: Block<RecordType = Self::RecordType>>(&self, block: B) -> bool;
    fn active_chain(&self) -> Result<Self::ChainType, ConsensusError>;
    fn branches(&mut self) -> Result<Self::BranchesType, ConsensusError>;
}

pub trait ConsensusRules<C: Chain> {
    fn merge(&mut self, branches: Vec<C>) -> Result<C, ConsensusError>;
}

pub trait ChainBranches<C: Chain, R: ConsensusRules<C>> {
    fn branches(&self) -> Result<Vec<C>, ConsensusError>;
    /// Merges the branches according to the rules and returns the resulting chain
    fn merge(&mut self, rules: R) -> Result<C, ConsensusError>;
}

pub enum ConsensusError {
    Custom(Box<dyn std::error::Error>),
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    Unspecified,
}

impl ConsensusError {
    pub fn custom(error: Box<dyn std::error::Error>) -> Self {
        ConsensusError::Custom(error)
    }
}
