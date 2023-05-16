use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    block::{Block, ChainedInstance, UnchainedInstance},
    chain::Chain,
    record::Record,
    Hash, SqliteBlock,
};

table! {
    blocks {
        id -> Integer,
        url -> Text,
        prevhash -> Text,

    }
}

#[cfg(feature = "chain")]

pub struct SqliteChain<X> {
    con: Arc<Mutex<SqliteConnection>>,
    _data: PhantomData<X>,
}

pub enum SqliteChainError {
    ConnectionError(ConnectionError),
}

impl From<ConnectionError> for SqliteChainError {
    fn from(value: ConnectionError) -> Self {
        SqliteChainError::ConnectionError(value)
    }
}

impl<X> SqliteChain<X> {
    fn gen_url() -> String {
        todo!()
    }
}

impl<X> SqliteChain<X> {
    pub fn new(url: &str) -> Result<Self, SqliteChainError> {
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con: Arc::new(Mutex::new(con)),
            _data: PhantomData,
        };
        Ok(val)
    }

    pub fn size(con: &mut SqliteConnection) -> usize {
        let r = blcks.select(id).count();

        let r = r.execute(&mut *con).unwrap();
        r as _
    }
}

use blocks::{dsl::blocks as blcks, id, url as paths};

impl<X: Record + Serialize + for<'a> Deserialize<'a> + 'static> Chain for SqliteChain<X> {
    type RecordType = X;

    type BlockType = SqliteBlock<X>;

    fn append(
        &self,
        data: &UnchainedInstance<Self::RecordType>,
    ) -> Result<ChainedInstance, crate::chain::ChainError> {
        let mut mut_ref = self.con.lock().unwrap();
        let size = Self::size(&mut *mut_ref);
        let prev_hash = {
            let block = self.block_at((size - 1) as _).unwrap();
            let mut res = block.map(|block| block.hash().unwrap());
            res.get_or_insert(Hash::default());
            res.unwrap()
        };
        let prev_hash_val = {
            let str = serde_json::to_string(&prev_hash);
            str.unwrap()
        };
        let gen_url = Self::gen_url();

        let smt = insert_into(blocks::table)
            .values((blocks::url.eq(&gen_url), blocks::prevhash.eq(prev_hash_val)));
        smt.execute(&mut *mut_ref).unwrap();
        let nonce = data.nonce();

        let (timestamp, hash) = SqliteBlock::build(&gen_url, data).unwrap();

        let new_instance = ChainedInstance::new(
            nonce.into(),
            (size as u64).into(),
            timestamp,
            hash,
            prev_hash,
            data.merkle_root().clone(),
        );
        Ok(new_instance)
    }

    fn block_at(&self, pos: u64) -> Result<Option<Self::BlockType>, crate::chain::ChainError> {
        let smt = blcks
            .select(paths)
            .find(pos as i32)
            .first::<String>(&mut *self.con.lock().unwrap())
            .unwrap();

        let res = SqliteBlock::new(&smt).unwrap();
        Ok(Some(res))
    }
}
