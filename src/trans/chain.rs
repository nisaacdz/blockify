use super::{record::Record, blocks::{BlockBuilder, Block}};

pub enum ChainErrors {
    IndexExceedsSize,
    BuilderSerializingError,
    BuilderDeserializingError,
}


pub trait Chain {
    fn append<'a, X: Record>(&self, data: &BlockBuilder<X>) -> Result<Block, ChainErrors>;
    fn get<'a, X: Record>(&self, pos: usize) -> Result<Block, ChainErrors>;
}
