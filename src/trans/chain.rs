use crate::{
    block::UnchainedInstance,
    data::Position,
    error::{DataBaseError, SerdeError},
};

use super::{
    block::{ChainedInstance, BlockError, PositionInstance},
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
pub trait Chain<R: Record>: Sized {
    type UnchainedInstanceType: UnchainedInstance<R>;
    /// The type of block that is stored in this chain.
    type ChainedInstanceType: ChainedInstance<R>;

    /// Appends an `UnchainedInstance` block to the chain.
    ///
    /// # Arguments
    ///
    /// * `block` - The `UnchainedInstance` to append to the block.
    ///
    /// # Returns
    ///
    /// - `Ok(PositionInstance)` If the operation succeeds
    /// - `Err(ChainError)` if the operation fails
    fn append(
        &mut self,
        block: &Self::UnchainedInstanceType,
    ) -> Result<PositionInstance, ChainError>;

    /// Gets a block from the chain by its position.
    ///
    /// Returns an error if the block is not found.
    fn block_at(&self, pos: Position) -> Result<Self::ChainedInstanceType, ChainError>;

    /// Gets a block from the chain by its chained instance.
    ///
    /// Returns an error if the block is not found.
    fn get(&self, b: PositionInstance) -> Result<Self::ChainedInstanceType, ChainError> {
        self.block_at(b.into_inner())
    }

    fn len(&self) -> Result<u64, ChainError>;

    fn last_block(&self) -> Result<Option<Self::ChainedInstanceType>, ChainError> {
        let last = match self.len()? {
            0 => return Ok(None),
            v => v.into(),
        };

        self.block_at(last).map(|value| Some(value))
    }
}
