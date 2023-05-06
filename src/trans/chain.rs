use crate::io::{DataBaseError, SerdeError};

use super::{
    blocks::{ChainedInstance, UnchainedInstance},
    image::{ChainImage, BlockImage},
    record::Record,
};

pub enum ChainError {
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    Unspecified,
}

pub trait Chain {
    fn append<X: Record>(&self, data: &UnchainedInstance<X>)
        -> Result<ChainedInstance, ChainError>;
    fn get<X: Record>(&self, pos: usize) -> Result<ChainedInstance, ChainError>;
    fn chain_image<X: Record, Y: ChainImage<X>>(&self) -> Y;
    fn block_image<X: Record, Y: BlockImage<X>>(&self, data: &ChainedInstance) -> Y;
}

