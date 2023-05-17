use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::data::TimeStamp;
use crate::{block::Block, record::Record};
use crate::{Hash, SqliteChainError};

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
pub struct SqliteBlock<X> {
    con: SqliteConnection,
    _data: PhantomData<X>,
}

#[derive(Debug)]
pub enum SqliteBlockError {
    ConnectionError(ConnectionError),
    ConnectionFailed,
}

impl Into<SqliteChainError> for SqliteBlockError {
    fn into(self) -> SqliteChainError {
        match self {
            SqliteBlockError::ConnectionError(ce) => SqliteChainError::ConnectionError(ce),
            Self::ConnectionFailed => SqliteChainError::ConnectionFailed,
        }
    }
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
        println!("url = {}", url);
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con,
            _data: PhantomData,
        };
        Ok(val)
    }

    fn create_tables(con: &mut SqliteConnection) -> Result<(), SqliteBlockError> {
        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY,
            values TEXT
        )",
        )
        .execute(con)
        .map_err(|_| SqliteBlockError::ConnectionFailed)?;

        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS metadata (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            hash TEXT,
            merkle TEXT,
            nonce TEXT
        )",
        )
        .execute(con)
        .map_err(|_| SqliteBlockError::ConnectionFailed)?;

        Ok(())
    }

    pub fn build(
        url: &str,
        instance: &UnchainedInstance<X>,
    ) -> Result<(TimeStamp, Hash), SqliteBlockError> {
        let mut val = Self::new(url)?;
        Self::create_tables(&mut val.con)?;
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
            smt.execute(&mut val.con).unwrap();
        }

        smt.execute(&mut val.con).unwrap();

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
    fn records(&mut self) -> Result<Box<[SignedRecord<X>]>, BlockError> {
        let res = rq
            .select(values)
            .load::<RecordValue<X>>(&mut self.con)
            .unwrap();
        let res = res
            .into_iter()
            .map(|record_json| record_json.into())
            .collect::<Vec<SignedRecord<X>>>();
        Ok(res.into_boxed_slice())
    }

    fn hash(&mut self) -> Result<Hash, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::hash)
            .first::<String>(&mut self.con)
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn merkle_root(&mut self) -> Result<crate::Hash, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::merkle)
            .first::<String>(&mut self.con)
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn nonce(&mut self) -> Result<u64, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::nonce)
            .first::<String>(&mut self.con)
            .unwrap();

        Ok(res.parse::<u64>().unwrap())
    }
}
