use diesel::prelude::*;
use std::marker::PhantomData;

use crate::{
    block::{BlockError, ChainedInstance, UnchainedInstance},
    data::{Nonce, Position, Timestamp},
    record::{Record, SignedRecord},
    Hash,
};

pub struct GenericBlock<R: Record> {
    _con: SqliteConnection,
    _dat: PhantomData<R>,
}

impl<R: Record> GenericBlock<R> {
    pub fn new(path: &str) -> Result<Self, BlockError> {
        todo!()
    }

    pub fn seal(&mut self) -> Result<(), BlockError> {
        todo!()
    }
}

impl<R: Record> UnchainedInstance<R> for GenericBlock<R> {
    fn append(&mut self, item: SignedRecord<R>) -> Result<(), BlockError> {
        todo!()
    }

    fn nonce(&self) -> Result<Nonce, BlockError> {
        todo!()
    }

    fn records(&self) -> Result<Vec<SignedRecord<R>>, BlockError> {
        todo!()
    }

    fn merkle_root(&self) -> Result<Hash, BlockError> {
        todo!()
    }
}

impl<R: Record> ChainedInstance<R> for GenericBlock<R> {
    fn records(&self) -> Result<Box<[SignedRecord<R>]>, BlockError> {
        todo!()
    }

    fn prev_hash(&self) -> Result<Hash, BlockError> {
        todo!()
    }

    fn position(&self) -> Result<Position, BlockError> {
        todo!()
    }

    fn hash(&self) -> Result<Hash, BlockError> {
        todo!()
    }

    fn merkle_root(&self) -> Result<Hash, BlockError> {
        todo!()
    }

    fn timestamp(&self) -> Result<Timestamp, BlockError> {
        todo!()
    }

    fn nonce(&self) -> Result<Nonce, BlockError> {
        todo!()
    }
}
