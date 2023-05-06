use crate::io::{DataBaseError, SerdeError};

use super::{
    blocks::{ChainedInstance, UnchainedInstance},
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
}
