use crate::io::{DataBaseError, SerdeError};

use super::{
    blocks::{BlockError, ChainedInstance, UnchainedInstance, Block},
    image::{BlockImage, ChainImage},
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
    fn get<X: Record, B: Block<X>>(&self, pos: usize) -> Result<B, ChainError>;
    fn chain_image<X: Record, Y: ChainImage<X>>(&self) -> Result<Y, ChainError>;
    fn block_image<X: Record, Y: BlockImage<X>>(
        &self,
        pos: &ChainedInstance,
    ) -> Result<Y, ChainError>;
}
