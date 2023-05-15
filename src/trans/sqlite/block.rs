use diesel::prelude::*;
use std::{marker::PhantomData, sync::Mutex, ops::DerefMut};

use crate::{block::Block, record::Record};

table! {
    records {
        id -> Integer,
        values -> Text,
    }
}

#[cfg(feature = "block")]
pub struct SqliteBlock<X> {
    con: Mutex<SqliteConnection>,
    _data: PhantomData<X>,
}

pub enum SqliteBlockError {
    ConnectionError(ConnectionError),
}

impl From<ConnectionError> for SqliteBlockError {
    fn from(value: ConnectionError) -> Self {
        SqliteBlockError::ConnectionError(value)
    }
}


impl<X> SqliteBlock<X> {
    pub fn new(url: &str) -> Result<Self, SqliteBlockError> {
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con: Mutex::new(con),
            _data: PhantomData,
        };
        Ok(val)
    }
}

use records::{values, dsl::records as rq};
use crate::record::SignedRecord;
use crate::block::BlockError;


impl<X: Record> Block<X> for SqliteBlock<X> {
    fn records(&self) -> Result<Box<[SignedRecord<X>]>, BlockError> {
        let mut mut_ref = self.con.lock().unwrap();
        let res = rq.select(values).load::<SignedRecord<X>>(mut_ref.deref_mut()).unwrap();
        Ok(res.into_boxed_slice())
    }

    fn hash(&self) -> Result<crate::Hash, crate::block::BlockError> {
        todo!()
    }

    fn merkle_root(&self) -> Result<crate::Hash, crate::block::BlockError> {
        todo!()
    }

    fn nonce(&self) -> Result<u64, crate::block::BlockError> {
        todo!()
    }
}
