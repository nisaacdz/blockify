use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{marker::PhantomData, sync::Mutex};

use crate::data::TimeStamp;
use crate::Hash;
use crate::{block::Block, record::Record};

table! {
    records {
        id -> Integer,
        values -> Text,
    }
}

table! {
    metadata {
        id -> Integer,
        timestamp -> Text,
        hash -> Text,
        merkle -> Text,
        nonce -> Text,
    }
}

#[cfg(feature = "block")]
pub struct SqliteBlock<X> {
    con: Arc<Mutex<SqliteConnection>>,
    _data: PhantomData<X>,
}

#[derive(Debug)]
pub enum SqliteBlockError {
    ConnectionError(ConnectionError),
}

impl std::fmt::Display for SqliteBlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<ConnectionError> for SqliteBlockError {
    fn from(value: ConnectionError) -> Self {
        SqliteBlockError::ConnectionError(value)
    }
}

impl<X: Record + Serialize> SqliteBlock<X> {
    pub fn new(url: &str) -> Result<Self, SqliteBlockError> {
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con: Arc::new(Mutex::new(con)),
            _data: PhantomData,
        };
        Ok(val)
    }

    pub fn build(url: &str, instance: &UnchainedInstance<X>) -> Result<(TimeStamp, Hash), ()> {
        let val = Self::new(url).unwrap();
        let mut mut_ref = val.con.lock().unwrap();
        use crate::data::ToTimeStamp;
        let current_time = chrono::Utc::now().to_timestamp();
        let timestamp = { serde_json::to_string(&current_time).unwrap() };
        let hash = crate::hash(&instance);
        let hash_str = {
            let hash = serde_json::to_string(&hash).unwrap();
            hash
        };

        let merkle = { serde_json::to_string(instance.merkle_root()).unwrap() };
        let nonce = instance.nonce().to_string();

        let smt = insert_into(metadata::table).values((
            metadata::timestamp.eq(timestamp),
            metadata::hash.eq(hash_str),
            metadata::merkle.eq(merkle),
            metadata::nonce.eq(nonce),
        ));

        for record in instance.records() {
            let smt = insert_into(records::table)
                .values(records::values.eq(serde_json::to_string(record).unwrap()));
            smt.execute(&mut *mut_ref).unwrap();
        }

        smt.execute(&mut *mut_ref).unwrap();

        Ok((current_time, hash))
    }
}

use crate::block::{BlockError, UnchainedInstance};
use crate::record::SignedRecord;
use records::{dsl::records as rq, values};

#[derive(Deserialize)]
struct RecordValue<X> {
    s: SignedRecord<X>,
}

impl<X: for<'a> Deserialize<'a>> Queryable<Text, Sqlite> for RecordValue<X> {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value = serde_json::from_str(&row)?;
        Ok(value)
    }
}

impl<X> From<RecordValue<X>> for SignedRecord<X> {
    fn from(value: RecordValue<X>) -> Self {
        value.s
    }
}

impl<X: Record + for<'a> Deserialize<'a> + 'static> Block for SqliteBlock<X> {
    type RecordType = X;
    fn records(&self) -> Result<Box<[SignedRecord<X>]>, BlockError> {
        let mut mut_ref = self.con.lock().unwrap();
        let res = rq
            .select(values)
            .load::<RecordValue<X>>(&mut *mut_ref)
            .unwrap();
        let res = res
            .into_iter()
            .map(|record_json| record_json.into())
            .collect::<Vec<SignedRecord<X>>>();
        Ok(res.into_boxed_slice())
    }

    fn hash(&self) -> Result<Hash, crate::block::BlockError> {
        let mut mut_ref = self.con.lock().unwrap();
        let res = metadata::table
            .select(metadata::hash)
            .first::<String>(&mut *mut_ref)
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn merkle_root(&self) -> Result<crate::Hash, crate::block::BlockError> {
        let mut mut_ref = self.con.lock().unwrap();
        let res = metadata::table
            .select(metadata::merkle)
            .first::<String>(&mut *mut_ref)
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn nonce(&self) -> Result<u64, crate::block::BlockError> {
        let mut mut_ref = self.con.lock().unwrap();
        let res = metadata::table
            .select(metadata::nonce)
            .first::<String>(&mut *mut_ref)
            .unwrap();

        Ok(res.parse::<u64>().unwrap())
    }
}
