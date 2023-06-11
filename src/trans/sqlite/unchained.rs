use std::marker::PhantomData;

use crate::{merkle::MerkleTree, record::Record, data::Metadata, block::UnchainedInstance};

pub struct GenericChain<R: Record> {
    _dat: PhantomData<R>,
}

impl<R: Record> UnchainedInstance<R> for GenericChain<R> {
    fn append(&mut self, item: crate::record::SignedRecord<R>) -> Result<(), crate::block::BlockError> {
        todo!()
    }

    fn nonce(&self) -> Result<crate::data::Nonce, crate::block::BlockError> {
        todo!()
    }

    fn records(&self) -> Result<Vec<crate::record::SignedRecord<R>>, crate::block::BlockError> {
        todo!()
    }

    fn merkle_root(&self) -> Result<crate::Hash, crate::block::BlockError> {
        todo!()
    }
}
