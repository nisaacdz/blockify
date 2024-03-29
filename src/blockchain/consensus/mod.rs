use crate::{
    block::{ChainedInstance, BlockError},
    chain::Chain,
    error::{DataBaseError, SerdeError},
    record::Record,
    Hash
};

pub mod puzzles;

pub trait ConsensusProtocol<R: Record> {
    type ChainedInstanceType: ChainedInstance<R>;
    type ChainType: Chain<R, ChainedInstanceType = Self::ChainedInstanceType>;
    type ConsensusRulesType: ConsensusRules<R, Self::ChainType>;
    type BranchesType: ChainBranches<R, Self::ChainType, Self::ConsensusRulesType>;
    fn validate<B: ChainedInstance<R>>(&self, block: B) -> bool;
    fn active_chain(&self) -> Result<Self::ChainType, ConsensusError>;
    fn branches(&mut self) -> Result<Self::BranchesType, ConsensusError>;
    fn hash_block(block: &Self::ChainedInstanceType) -> Result<Hash, BlockError>;
}

pub trait ConsensusRules<R: Record, C: Chain<R>> {
    fn merge(&mut self, branches: Vec<C>) -> Result<C, ConsensusError>;
}

pub trait ChainBranches<R: Record, C: Chain<R>, X: ConsensusRules<R, C>> {
    fn branches(&self) -> Result<Vec<C>, ConsensusError>;
    /// Merges the branches according to the rules and returns the resulting chain
    fn merge(&mut self, rules: X) -> Result<C, ConsensusError>;
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
