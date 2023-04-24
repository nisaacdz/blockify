use super::{record::Record, blocks::{ChainedBlock, Block}};

pub enum ChainErrors {
    IndexExceedsSize,
    BuilderSerializingError,
    BuilderDeserializingError,
}


pub trait Chain {
    fn append<'a, X: Record>(&self, data: &Block<X>) -> Result<ChainedBlock, ChainErrors>;
    fn get<'a, X: Record>(&self, pos: usize) -> Result<ChainedBlock, ChainErrors>;
}
