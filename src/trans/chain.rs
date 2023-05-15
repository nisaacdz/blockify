use crate::io::{DataBaseError, SerdeError};

use super::{
    block::{BlockError, ChainedInstance, UnchainedInstance, Block},
    record::Record,
};

pub enum ChainError {
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    Unspecified,
}

impl From<BlockError> for ChainError {
    fn from(value: BlockError) -> Self {
        match value {
            BlockError::SerdeError(v) => ChainError::SerdeError(v),
            BlockError::DataBaseError(u) => ChainError::DataBaseError(u),
            BlockError::Unspecified => ChainError::Unspecified,
        }
    }
}

pub trait Chain {
    fn append<X: Record>(&self, data: &UnchainedInstance<X>)
        -> Result<ChainedInstance, ChainError>;
    fn block_at<X: Record, B: Block<X>>(&self, pos: u64) -> Result<B, ChainError>;
    fn get<X: Record, B: Block<X>>(&self, b: &ChainedInstance) -> Result<B, ChainError> {
        let pos = b.position();
        let block = self.block_at(pos)?;
        Ok(block)
    }
}
