use super::{
    blocks::{UnchainedInstance, ChainedInstance},
    record::Record,
};

pub trait Chain {
    type ErrorType;
    fn append<X: Record>(&self, data: &UnchainedInstance<X>) -> Result<ChainedInstance, Self::ErrorType>;
    fn get<X: Record>(&self, pos: usize) -> Result<ChainedInstance, Self::ErrorType>;
}
