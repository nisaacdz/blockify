use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, fmt::Debug};

use crate::{
    block::{Block, ChainedInstance, UnchainedInstance},
    chain::{Chain, ChainError},
    io::DataBaseError,
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

pub struct SqliteChain<X> {
    con: SqliteConnection,
    url: String,
    _data: PhantomData<X>,
}

#[derive(Debug)]
pub enum SqliteChainError {
    ConnectionError(ConnectionError),
    ConnectionFailed,
}

impl From<ConnectionError> for SqliteChainError {
    fn from(value: ConnectionError) -> Self {
        SqliteChainError::ConnectionError(value)
    }
}

impl<X> SqliteChain<X> {
    fn gen_url(url: &str, feed: i64) -> String {
        format!("{}block{}.db", url, feed)
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
            con,
            _data: PhantomData,
        };

        Ok(value)
    }

    fn create_table(con: &mut SqliteConnection) -> Result<(), SqliteChainError> {
        diesel::sql_query(
            "
        CREATE TABLE IF NOT EXISTS blocks (
            id INTEGER PRIMARY KEY,
            url TEXT,
            prevhash TEXT
        )
        ",
        )
        .execute(con)
        .map_err(|_| SqliteChainError::ConnectionFailed)?;

        Ok(())
    }

    pub fn size(con: &mut SqliteConnection) -> i64 {
        let c: i64 = blocks::table.count().get_result(con).unwrap();
        c
    }
}

impl<X: Record + Serialize + for<'a> Deserialize<'a> + 'static> Chain for SqliteChain<X> {
    type RecordType = X;

    type BlockType = SqliteBlock<X>;

    fn append(
        &mut self,
        data: &UnchainedInstance<Self::RecordType>,
    ) -> Result<ChainedInstance, crate::chain::ChainError> {
        let size = Self::size(&mut self.con);
        let prev_hash = {
            if size == 0 {
                Hash::default()
            } else {
                let mut block = self.block_at((size - 1) as _)?;
                block.hash()?
            }
        };
        let prev_hash_val = {
            let str = serde_json::to_string(&prev_hash);
            str.unwrap()
        };
        let gen_url = Self::gen_url(&self.url, size);

        let smt = insert_into(blocks::table)
            .values((blocks::url.eq(&gen_url), blocks::prevhash.eq(prev_hash_val)));
        smt.execute(&mut self.con).unwrap();
        let nonce = data.nonce();

        let (timestamp, hash) = match SqliteBlock::build(&gen_url, data) {
            Ok(v) => v,
            Err(v) => panic!("block could not be built: Reason = {v}"),
        };

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

    fn block_at(&mut self, pos: u64) -> Result<Self::BlockType, ChainError> {
        let url: String = blocks::table.select(blocks::url)
        .filter(blocks::id.eq(pos as i32))
        .first(&mut self.con).map_err(|_| ChainError::AbsentValue)?;

        let block = SqliteBlock::new(&url).map_err(|_| ChainError::DataBaseError(DataBaseError::ConnectionCannotEstablish))?;
        
        Ok(block)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::UnchainedInstance,
        chain::Chain,
        data::MetaData,
        record::{Record, SignedRecord},
        SqliteChain,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Record, Clone, Serialize, Deserialize)]
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
        let chain_url = "src/trans/sqlite/";
        let datas1 = vec!["abcd", "efgh", "ijkl"];
        let datas2 = vec!["mnop", "qrst", "uvwx"];
        let keypair = crate::generate_ed25519_key_pair();
        let records1 = datas1
            .into_iter()
            .map(|w| Vote::new(w).record(keypair.clone(), MetaData::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<SignedRecord<Vote>>>();
        let records2 = datas2
            .into_iter()
            .map(|w| Vote::new(w).record(keypair.clone(), MetaData::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<SignedRecord<Vote>>>();

        let mut block1 = UnchainedInstance::new(MetaData::empty());
        let mut block2 = UnchainedInstance::new(MetaData::empty());

        for record in records1 {
            block1.push(record);
        }

        for record in records2 {
            block2.push(record);
        }

        let mut chain =
            SqliteChain::new(chain_url).expect("sqlite connection cannot be established");
        chain.append(&block1).expect("block1 append erred");
        chain.append(&block2).expect("block2 append erred");
    }
}
