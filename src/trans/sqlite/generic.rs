use diesel::prelude::*;
use serde::Serialize;
use std::marker::PhantomData;

use crate::{
    block::{BlockError, ChainedInstance, UnchainedInstance},
    data::{Nonce, Position, Timestamp},
    record::{Record, Records, SignedRecord},
    Hash, WrapperMut,
};

// records others
// Metadata
// Position
// MerkleTree
// Nonce
// Prev-hash
// Hash
// seal

table! {
    records {
        id -> Integer,
        jsonvalues -> Text,
    }
}

pub struct GenericBlock<R> {
    con: WrapperMut<SqliteConnection>,
    _data: PhantomData<R>,
}

pub enum GenericBlockError {
    ConnectionError(ConnectionError),
    QueryNotExecuted,
}

impl From<ConnectionError> for GenericBlockError {
    fn from(value: ConnectionError) -> Self {
        GenericBlockError::ConnectionError(value)
    }
}

impl<R> GenericBlock<R> {
    pub fn new(url: &str) -> Result<Self, GenericBlockError> {
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con: WrapperMut::new(con),
            _data: PhantomData,
        };
        Ok(val)
    }

    pub fn build_tables(con: &mut SqliteConnection) -> Result<(), GenericBlockError> {
        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY,
            jsonvalues TEXT
        )
        ",
        )
        .execute(con)
        .map_err(|_| GenericBlockError::QueryNotExecuted)?;
        Ok(())
    }

    pub fn update_merkle(&mut self, _: Hash, _: Position) -> Result<(), BlockError> {
        todo!()
    }

    pub fn recompute_hash(&self, _record_hash: &Hash) -> Result<Hash, BlockError> {
        todo!()
    }

    pub fn seal(&mut self) -> Result<(), BlockError> {
        todo!()
    }

    pub fn len(&self) -> Result<u64, BlockError> {
        todo!()
    }
}

impl<R: Record + Serialize> UnchainedInstance<R> for GenericBlock<R> {
    fn append(&mut self, item: SignedRecord<R>) -> Result<(), BlockError> {
        let sz = self.len()?;

        
        let _record = serde_json::to_string(&item).unwrap();
        let _hash = serde_json::to_string(item.hash()).unwrap();
        
        let hash = self.recompute_hash(item.hash())?;
        self.update_merkle(hash, sz.into())?;

        Ok(())
    }

    fn nonce(&self) -> Result<Nonce, BlockError> {
        todo!()
    }

    fn records(&self) -> Result<Records<R>, BlockError> {
        todo!()
    }

    fn merkle_root(&self) -> Result<Hash, BlockError> {
        todo!()
    }
}

impl<R: Record> ChainedInstance<R> for GenericBlock<R> {
    fn records(&self) -> Result<Records<R>, BlockError> {
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
