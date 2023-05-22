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
            BlockError::NotValid(_) => unimplemented!(),
        }
    }
}

/// A chain is a collection of blocks.
///
/// The `Chain` trait provides methods for adding blocks to the chain, getting blocks from the chain, and validating the chain.
pub trait Chain {
    /// The type of record that is stored in the blocks in this chain.
    type RecordType: Record;

    /// The type of block that is stored in this chain.
    type BlockType: Block<RecordType = Self::RecordType>;

    /// Appends a block to the chain.
    ///
    /// Returns an error if the block is not valid.
    fn append(
        &mut self,
        data: &UnchainedInstance<Self::RecordType>,
    ) -> Result<ChainedInstance, ChainError>;

    /// Gets a block from the chain by its position.
    ///
    /// Returns an error if the block is not found.
    fn block_at(&self, pos: Position) -> Result<Self::BlockType, ChainError>;

    /// Gets a block from the chain by its chained instance.
    ///
    /// Returns an error if the block is not found.
    fn get(&self, b: &ChainedInstance) -> Result<Self::BlockType, ChainError> {
        let pos = b.position();
        let block = self.block_at(pos)?;
        Ok(block)
    }
}

