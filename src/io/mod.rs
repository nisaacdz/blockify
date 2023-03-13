use std::slice::Iter;

use crate::{
    axs::unit::Units,
    errs::BlockifyError,
    refs::{Range, TimeStamp, ToTimeStamp},
    trans::{blocks::Block, record::Record},
};

pub trait BlockBaseInsertable<R: RecordBaseInsertable<X>, X: Record> {
    fn name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn records(&self) -> Iter<R>;

    fn merke_root(&self) -> &[u8];

    fn insertion(
        &self,
        hash: &[u8],
        prev: &[u8],
        range: Range,
        timestamp: TimeStamp,
    ) -> Vec<String>;

    fn size(&self) -> u64;

    fn generate(
        &self,
        hash: Vec<u8>,
        prev_hash: Vec<u8>,
        timestamp: TimeStamp,
        range: Range,
        position: u64,
    ) -> Block;
}

pub trait RecordBaseInsertable<X: Record> {
    fn name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn record(&self) -> &X;
}

pub trait BlockBase<B: BlockBaseInsertable<R, X>, R: RecordBaseInsertable<X>, X: Record> {
    fn create_table(&mut self, table_name: &str, colums: &[&str]) -> Result<(), BlockifyError>;

    fn number_of_blocks(&self) -> Option<u64> {
        self.count_rows(B::name())
    }

    fn number_of_records(&self) -> Option<u64> {
        self.count_rows(R::name())
    }

    fn count_rows(&self, table_name: &str) -> Option<u64>;

    fn table_exists(&self, table_name: &str) -> bool;

    fn insert(&mut self, block: B) -> Result<Block, BlockifyError> {
        let begin = match self.count_rows(R::name()) {
            Some(v) => v,
            _ => return Err(BlockifyError::new(&format!("No Such Table: {}", R::name()))),
        };

        let end = begin + block.size() - 1;

        let range = Range::new(begin, end);

        let prev_hash = self.prev_hash_or_default()?;
        for record in block.records() {
            self.insert_record(record)?;
        }

        let timestamp = chrono::Local::now().naive_utc().to_local_timestamp();

        let position = match self.count_rows(B::name()) {
            Some(v) => v,
            _ => return Err(BlockifyError::new(&format!("No Such Table: {}", B::name()))),
        };

        let hash = crate::sec::sha_from_2(&prev_hash, block.merke_root());

        self.insert_block(&block.insertion(&hash, &prev_hash, range, timestamp))?;

        Ok(block.generate(hash, prev_hash, timestamp, range, position))
    }

    fn insert_record(&self, record: &R) -> Result<(), BlockifyError>;
    fn insert_block(&self, insertion: &[String]) -> Result<(), BlockifyError>;

    fn prev_hash_or_default(&self) -> Result<Vec<u8>, BlockifyError>;

    fn view_records(&self, range: Range) -> Result<Vec<R>, BlockifyError>;
}

pub trait UnitBase {
    fn units(&self, peer: &[u8]) -> Option<Vec<Units>>;
}

pub trait MemPool {
    fn pool_raw(&self) -> Result<Vec<String>, BlockifyError>;
    fn append(&mut self, record: &str) -> Result<(), BlockifyError>;
    fn poll(&self) -> Result<String, BlockifyError>;
    fn size(&self) -> u64;
}

pub trait NodeRecord {
    fn records_of(&self, peer: &[u8]) -> Vec<String>;
}
