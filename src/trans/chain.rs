use super::{
    blocks::{ChainedInstance, UnchainedInstance},
    record::Record,
};

pub enum ChainError {
    SerializationError,
    DeserializationError,
    CouldNotEstablishDatabaseConnection,
    NoSuchEntityInDatabase(Option<&'static str>),
    UknownError,
}

pub trait Chain {
    fn append<X: Record>(&self, data: &UnchainedInstance<X>)
        -> Result<ChainedInstance, ChainError>;
    fn get<X: Record>(&self, pos: usize) -> Result<ChainedInstance, ChainError>;
}
