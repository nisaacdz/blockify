use super::{record::Record, blocks::{ChainedBlock, Block}};

pub enum ChainErrors {
    IndexExceedsSize,
    BuilderSerializingError,
    BuilderDeserializingError,
}


pub trait Chain {
    fn append<X: Record>(&self, data: &Block<X>) -> Result<ChainedBlock, ChainErrors>;
    fn get<X: Record>(&self, pos: usize) -> Result<ChainedBlock, ChainErrors>;
}
