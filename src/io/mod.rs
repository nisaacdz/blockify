use std::slice::Iter;

use crate::{*, errs::BlockBaseErrs, trans::{blocks::Block, record::Record}};

pub trait BlockBaseInsertable<R: RecordBaseInsertable<X>, X: Record> {
    fn name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn records(&self) -> Iter<R>;

    fn insertion(
        &self,
        hash: &[u8],
        prev: &[u8],
        range: Range,
        timestamp: TimeStamp,
    ) -> Vec<String>;

    fn size(&self) -> u64;

    fn hash(&self) -> &[u8];

    fn generate(
        &self,
        hash: Vec<u8>,
        prev_hash: Vec<u8>,
        timestamp: TimeStamp,
        range: Range,
        position: u64,
    ) -> Block<X>;
}

/// This considers the implementing struct fit for inserting into the blockchain
/// as an individual record
pub trait RecordBaseInsertable<X: Record> {
    fn name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn record(&self) -> &X;
}

pub trait BlockBase<B: BlockBaseInsertable<R, X>, R: RecordBaseInsertable<X>, X: Record> {
    /// Creates a new table with the given name and columns, if table doesn't already exist.
    /// `ALL` columns are stored as text
    /// `Ok(())` value indicates success whiles `BlockBaseErrs` indicate different failures
    fn create_table(&mut self, table_name: &str, colums: &[&str]) -> Result<(), BlockBaseErrs>;

    fn number_of_blocks(&self) -> Option<u64> {
        self.count_rows(B::name())
    }

    fn number_of_records(&self) -> Option<u64> {
        self.count_rows(R::name())
    }

    ///
    fn count_rows(&self, table_name: &str) -> Option<u64>;

    /// Checks if there is a table with name `table_name` present in the database
    fn table_exists(&self, table_name: &str) -> bool;

    /// Inserts the given `BlockBaseInsertable` item into its table in the database
    fn insert(&mut self, block: B) -> Result<Block<X>, BlockBaseErrs> {
        let begin = match self.count_rows(R::name()) {
            Some(v) => v,
            _ => return Err(BlockBaseErrs::NoSuchTable(R::name().to_owned())),
        };

        let end = begin + block.size() - 1;

        let range = Range::new(begin, end);

        let prev_hash = self.prev_hash_or_default()?;
        let hash = sec::sha_from_2(&prev_hash, block.hash());

        for record in block.records() {
            self.insert_record(record)?;
        }

        let timestamp = chrono::Local::now().naive_utc().to_local_timestamp();

        let position = match self.count_rows(B::name()) {
            Some(v) => v,
            _ => return Err(BlockBaseErrs::NoSuchTable(B::name().to_owned())),
        };

        self.insert_block(&block.insertion(&hash, &prev_hash, range, timestamp))?;

        Ok(block.generate(hash, prev_hash, timestamp, range, position))
    }

    fn insert_record(&self, record: &R) -> Result<(), BlockBaseErrs>;
    fn insert_block(&self, insertion: &[String]) -> Result<(), BlockBaseErrs>;

    fn prev_hash_or_default(&self) -> Result<Vec<u8>, BlockBaseErrs>;
}

pub trait UnitBase {
    fn units_for(&self, entity: &[u8]) -> Option<Vec<Unit>>;
}
