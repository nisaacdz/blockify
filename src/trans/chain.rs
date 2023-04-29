use super::{
    blocks::{Block, ChainedInstance},
    record::Record,
};

pub enum ChainErrors {
    IndexExceedsSize,
    BuilderSerializingError,
    BuilderDeserializingError,
}

pub trait Chain {
    fn append<X: Record>(&self, data: &Block<X>) -> Result<ChainedInstance, ChainErrors>;
    fn get<X: Record>(&self, pos: usize) -> Result<ChainedInstance, ChainErrors>;
}
