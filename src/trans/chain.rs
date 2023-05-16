use crate::io::{DataBaseError, SerdeError};

use super::{
    block::{Block, BlockError, ChainedInstance, UnchainedInstance},
    record::Record,
};

#[derive(Debug, Clone, Copy)]
pub enum ChainError {
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    AbsentPosition,
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
    type RecordType: Record;
    type BlockType: Block<RecordType = Self::RecordType>;
    fn append(
        &self,
        data: &UnchainedInstance<Self::RecordType>,
    ) -> Result<ChainedInstance, ChainError>;
    fn block_at(&self, pos: u64) -> Result<Option<Self::BlockType>, ChainError>;
    fn get(&self, b: &ChainedInstance) -> Result<Self::BlockType, ChainError> {
        let pos = b.position();
        let block = self.block_at(pos)?.ok_or(ChainError::AbsentPosition)?;
        Ok(block)
    }
}
