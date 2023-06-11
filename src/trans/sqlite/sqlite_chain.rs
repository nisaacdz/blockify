use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

use crate::{
    block::{Block, LocalInstance, PositionInstance, UnchainedInstance},
    chain::{Chain, ChainError},
    data::{Position, ToTimestamp},
    error::{DataBaseError, SerdeError},
    record::Record,
    Hash, SqliteBlock, TempInstance,
};

use super::WrapperMut;

table! {
    blocks {
        id -> Integer,
        block -> Text,
    }
}

pub struct SqliteChain<X> {
    con: WrapperMut<SqliteConnection>,
    url: String,
    _data: PhantomData<X>,
}

#[derive(Debug)]
pub enum SqliteChainError {
    ConnectionError(ConnectionError),
    SerdeError(SerdeError),
    ConnectionFailed,
}

impl From<ConnectionError> for SqliteChainError {
    fn from(value: ConnectionError) -> Self {
        SqliteChainError::ConnectionError(value)
    }
}

impl<X> SqliteChain<X> {
    fn gen_url(url: &str, feed: i64) -> String {
        format!("{}block{}.db", url, feed + 1)
    }
}

impl<X> SqliteChain<X> {
    pub fn new(url: &str) -> Result<Self, SqliteChainError> {
        assert!(url.ends_with('/'));
        let basic = format! {"{url}chain.db"};
        let mut con = SqliteConnection::establish(&basic)
            .map_err(|e| SqliteChainError::ConnectionError(e))?;

        Self::create_table(&mut con)?;

        let value = Self {
            url: url.to_owned(),
            con: WrapperMut::new(con),
            _data: PhantomData,
        };

        Ok(value)
    }

    fn create_table(con: &mut SqliteConnection) -> Result<(), SqliteChainError> {
        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS blocks (
            id INTEGER PRIMARY KEY,
            block TEXT
        )
        ",
        )
        .execute(con)
        .map_err(|_| SqliteChainError::ConnectionFailed)?;

        Ok(())
    }

    pub fn size(con: &mut SqliteConnection) -> Result<u64, DataBaseError> {
        let c = match blocks::table.count().get_result::<i64>(con) {
            Ok(v) => v as u64,
            Err(_) => return Err(DataBaseError::NoSuchTable),
        };
        Ok(c as _)
    }
}

impl<X: Clone + Record + Serialize + for<'a> Deserialize<'a> + 'static> Chain<X>
    for SqliteChain<X>
{
    type UnchainedInstanceType = LocalInstance<X>;

    type BlockType = SqliteBlock<X>;

    fn append(
        &mut self,
        block: &Self::UnchainedInstanceType,
    ) -> Result<PositionInstance, ChainError> {
        let size = Self::size(self.con.get_mut()).map_err(|e| ChainError::DataBaseError(e))?;

        let nonce = block.nonce().unwrap();

        let position = (size + 1).into();

        let timestamp = chrono::Utc::now().to_timestamp();

        let merkle_root = block.merkle_root().unwrap().clone();

        let prev_hash = match self.block_at(size.into()) {
            Err(ChainError::AbsentValue) => Hash::default(),
            other => {
                let other = other?;
                other.hash()?
            }
        };

        let hash = crate::hash_block(&block, &prev_hash, &timestamp, &position);

        let chained = TempInstance::new(nonce, position, timestamp, hash, prev_hash, merkle_root);

        let gen_url = Self::gen_url(&self.url, size as _);

        let smt = insert_into(blocks::table).values(blocks::block.eq(&gen_url));
        smt.execute(self.con.get_mut()).unwrap();

        SqliteBlock::build(&gen_url, &*block.records().unwrap(), &chained).unwrap();

        Ok(PositionInstance::new(position))
    }

    fn block_at(&self, pos: Position) -> Result<Self::BlockType, ChainError> {
        if pos.pos == 0 {
            return Err(ChainError::AbsentValue);
        }

        let url: String = blocks::table
            .select(blocks::block)
            .filter(blocks::id.eq(pos.pos as i32))
            .first(self.con.get_mut())
            .map_err(|_| ChainError::AbsentValue)?;

        let block = SqliteBlock::new(&url)
            .map_err(|_| ChainError::DataBaseError(DataBaseError::ConnectionCannotEstablish))?;

        Ok(block)
    }

    fn len(&self) -> Result<u64, ChainError> {
        Self::size(self.con.get_mut()).map_err(|e| ChainError::DataBaseError(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as blockify, block::LocalInstance};

    use blockify::{
        block::{Block, UnchainedInstance},
        chain::Chain,
        data::Metadata,
        record::{Record, SignedRecord},
        SqliteChain,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Record, Clone, Serialize, Deserialize, PartialEq)]
    struct Vote {
        data: String,
    }

    impl Vote {
        pub fn new(data: &str) -> Self {
            Vote { data: data.into() }
        }
    }

    #[test]
    fn test_block() {
        let chain_url = "target2/tests/votestoringstring/";
        std::fs::create_dir_all(chain_url).expect("could not create chain_url");
        let datas1 = vec!["abcd", "efgh", "ijkl"];
        let datas2 = vec!["mnop", "qrst", "uvwx"];
        let keypair = crate::generate_ed25519_keypair();
        let records1 = datas1
            .into_iter()
            .map(|w| Vote::new(w).record(keypair.clone(), Metadata::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<SignedRecord<Vote>>>();
        let records2 = datas2
            .into_iter()
            .map(|w| Vote::new(w).record(keypair.clone(), Metadata::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<SignedRecord<Vote>>>();

        let mut builder1 = LocalInstance::new(Metadata::empty(), 0);
        let mut builder2 = LocalInstance::new(Metadata::empty(), 1);

        for record in records1 {
            builder1.push(record);
        }

        for record in records2 {
            builder2.push(record);
        }

        let mut chain =
            SqliteChain::new(chain_url).expect("sqlite connection cannot be established");
        let instance1 = chain.append(&builder1).expect("builder1 append erred");
        let instance2 = chain.append(&builder2).expect("builder2 append erred");

        let block1 = instance1.block(&chain).expect("couldn't retrieve block1");
        let block2 = instance2.block(&chain).expect("couldn't retrieve block2");

        let records_from_block1 = block1
            .records()
            .expect("couldn't retrieve records from block1");
        assert_eq!(
            builder1.records().unwrap().as_slice(),
            &*records_from_block1
        );

        let records_from_block2 = block2
            .records()
            .expect("couldn't retrieve records from block2");
        assert_eq!(
            builder2.records().unwrap().as_slice(),
            &*records_from_block2
        );
    }
}
