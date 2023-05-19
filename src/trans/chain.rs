use crate::{
    data::Position,
    io::{DataBaseError, SerdeError},
};

use super::{
    block::{Block, BlockError, ChainedInstance, UnchainedInstance},
    record::Record,
};

#[derive(Debug, Clone, Copy)]
pub enum ChainError {
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    AbsentValue,
    Unspecified,
}

impl From<BlockError> for ChainError {
    fn from(value: BlockError) -> Self {
        match value {
            BlockError::SerdeError(v) => ChainError::SerdeError(v),
            BlockError::DataBaseError(u) => ChainError::DataBaseError(u),
            BlockError::Unspecified => ChainError::Unspecified,
            BlockError::NotValid => unimplemented!(),
        }
    }
}

pub trait Chain {
    type RecordType: Record;
    type BlockType: Block<RecordType = Self::RecordType>;
    fn append(
        &mut self,
        data: &UnchainedInstance<Self::RecordType>,
    ) -> Result<ChainedInstance, ChainError>;
    fn block_at(&mut self, pos: Position) -> Result<Self::BlockType, ChainError>;
    fn get(&mut self, b: &ChainedInstance) -> Result<Self::BlockType, ChainError> {
        let pos = b.position();
        let block = self.block_at(pos)?;
        Ok(block)
    }
}
