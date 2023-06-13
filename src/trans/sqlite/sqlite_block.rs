use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::data::{Nonce, Position, Timestamp};
use crate::error::SerdeError;
use crate::{
    block::ChainedInstance,
    record::{Record, Records},
};
use crate::{Hash, SqliteChainError, TempInstance};

use super::WrapperMut;

table! {
    records {
        id -> Integer,
        jsonvalues -> Text,
    }
}

table! {
    metadata {
        id -> Integer,
        timestamp -> Text,
        hash -> Text,
        merkle_root -> Text,
        nonce -> Text,
        prev_hash -> Text,
        position -> Text,
    }
}

pub struct SqliteBlock<X> {
    con: WrapperMut<SqliteConnection>,
    _data: PhantomData<X>,
}

#[derive(Debug)]
pub enum SqliteBlockError {
    ConnectionError(ConnectionError),
    SerdeError(SerdeError),
    ConnectionFailed,
}

impl Into<SqliteChainError> for SqliteBlockError {
    fn into(self) -> SqliteChainError {
        match self {
            SqliteBlockError::ConnectionError(ce) => SqliteChainError::ConnectionError(ce),
            Self::ConnectionFailed => SqliteChainError::ConnectionFailed,
            Self::SerdeError(sd) => SqliteChainError::SerdeError(sd),
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
        let con = SqliteConnection::establish(url)?;
        let val = Self {
            con: WrapperMut::new(con),
            _data: PhantomData,
        };
        Ok(val)
    }

    fn create_tables(con: &mut SqliteConnection) -> Result<(), SqliteBlockError> {
        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY,
            jsonvalues TEXT
        )
        ",
        )
        .execute(con)
        .map_err(|_| SqliteBlockError::ConnectionFailed)?;

        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS metadata (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            hash TEXT,
            merkle_root TEXT,
            nonce TEXT,
            prev_hash TEXT,
            position TEXT
        )",
        )
        .execute(con)
        .map_err(|_| SqliteBlockError::ConnectionFailed)?;

        Ok(())
    }

    pub fn build(
        url: &str,
        records: &[SignedRecord<X>],
        cc: &TempInstance,
    ) -> Result<Self, SqliteBlockError> {
        let TempInstance {
            nonce,
            position,
            hash,
            prev_hash,
            merkle_root,
            timestamp,
        } = cc;
        let val = Self::new(url)?;
        Self::create_tables(val.con.get_mut())?;

        let timestamp = serde_json::to_string(timestamp).unwrap();

        let hash = serde_json::to_string(hash).unwrap();

        let prev_hash = serde_json::to_string(prev_hash).unwrap();

        let merkle_root = { serde_json::to_string(merkle_root).unwrap() };

        let nonce = { serde_json::to_string(nonce).unwrap() };

        let position = serde_json::to_string(position).unwrap();

        let smt = diesel::insert_into(metadata::table).values((
            metadata::timestamp.eq(timestamp),
            metadata::hash.eq(hash),
            metadata::merkle_root.eq(merkle_root),
            metadata::nonce.eq(nonce),
            metadata::prev_hash.eq(prev_hash),
            metadata::position.eq(position),
        ));

        for record in records {
            let smt = diesel::insert_into(records::table)
                .values(records::jsonvalues.eq(serde_json::to_string(record).unwrap()));
            smt.execute(val.con.get_mut()).unwrap();
        }

        smt.execute(val.con.get_mut()).unwrap();

        Ok(val)
    }
}

use crate::block::BlockError;
use crate::record::SignedRecord;
use records::dsl::records as rq;

#[derive(Deserialize)]
struct RecordValue<X> {
    s: SignedRecord<X>,
}

impl<X> RecordValue<X> {
    fn new(s: SignedRecord<X>) -> Self {
        Self { s }
    }
}

impl<X: for<'a> Deserialize<'a>> Queryable<Text, Sqlite> for RecordValue<X> {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value = serde_json::from_str(&row)?;
        Ok(RecordValue::new(value))
    }
}

impl<X> From<RecordValue<X>> for SignedRecord<X> {
    fn from(value: RecordValue<X>) -> Self {
        value.s
    }
}

impl<X: Record + for<'a> Deserialize<'a> + 'static> ChainedInstance<X> for SqliteBlock<X> {
    fn records(&self) -> Result<Records<X>, BlockError> {
        let res = rq
            .select(records::jsonvalues)
            .load::<RecordValue<X>>(self.con.get_mut())
            .unwrap();
        let res = res
            .into_iter()
            .map(|record_val| record_val.into())
            .collect::<Vec<SignedRecord<X>>>();
        Ok(res.into())
    }

    fn hash(&self) -> Result<Hash, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::hash)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn merkle_root(&self) -> Result<crate::Hash, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::merkle_root)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn nonce(&self) -> Result<Nonce, crate::block::BlockError> {
        let res = metadata::table
            .select(metadata::nonce)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Nonce>(&res).unwrap();
        Ok(res)
    }

    fn prev_hash(&self) -> Result<Hash, BlockError> {
        let res = metadata::table
            .select(metadata::prev_hash)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Hash>(&res).unwrap();
        Ok(res)
    }

    fn position(&self) -> Result<Position, BlockError> {
        let res = metadata::table
            .select(metadata::position)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Position>(&res).unwrap();
        Ok(res)
    }

    fn timestamp(&self) -> Result<Timestamp, BlockError> {
        let res = metadata::table
            .select(metadata::timestamp)
            .first::<String>(self.con.get_mut())
            .unwrap();
        let res = serde_json::from_str::<Timestamp>(&res).unwrap();
        Ok(res)
    }
}
