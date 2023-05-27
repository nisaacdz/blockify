use crate::{
    data::Position,
    io::{DataBaseError, SerdeError},
};

use super::{
    block::{Block, BlockError, ChainedInstance, UnchainedInstance},
    record::Record,
};

/// The types of error that can occur in operations associated with the `Chain` trait
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
pub trait Chain: Sized {
    /// The type of record that is stored in the blocks in this chain.
    type RecordType: Record;

    type ChainedInstanceType: ChainedInstance<Self>;

    /// The type of block that is stored in this chain.
    type BlockType: Block<RecordType = Self::RecordType>;

    /// Appends an `UnchainedInstance` block to the chain.
    ///
    /// # Arguments
    ///
    /// * `block` - The `UnchainedInstance` to append to the block.
    ///
    /// # Returns
    ///
    /// - `Ok(ChainedInstance)` If the operation succeeds
    /// - `Err(ChainError)` if the operation fails
    fn append(
        &mut self,
        block: &UnchainedInstance<Self::RecordType>,
    ) -> Result<Self::ChainedInstanceType, ChainError>;

    /// Gets a block from the chain by its position.
    ///
    /// Returns an error if the block is not found.
    fn block_at(&self, pos: Position) -> Result<Self::BlockType, ChainError>;

    /// Gets a block from the chain by its chained instance.
    ///
    /// Returns an error if the block is not found.
    fn get(&self, b: Self::ChainedInstanceType) -> Result<Self::BlockType, ChainError> {
        let res = b.block(self)?;
        Ok(res)
    }
}
