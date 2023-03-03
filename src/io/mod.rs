use std::slice::Iter;

use crate::{errs::BlockBaseErrs, block::Block, record::Record};

pub trait BlockBaseInsertable<R: RecordBaseInsertable<X>, X: Record> {
    fn get_name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn get_rows(&self) -> Iter<R>;

    fn size(&self) -> u64;
}


/// This considers the implementing struct fit for inserting into the blockchain 
/// as an individual record
pub trait RecordBaseInsertable<X: Record> {
    fn get_name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn get_record(&self) -> &X;
}

pub trait BlockBase<B: BlockBaseInsertable<R, X>, R: RecordBaseInsertable<X>, X: Record> {
    /// Creates a new table with the given name and columns, if table doesn't already exist.
    /// `ALL` columns are stored as text
    /// `Ok(())` value indicates success whiles `BlockBaseErrs` indicate different failures
    fn create_table(&mut self, table_name: &str, colums: &[&str]) -> Result<(), BlockBaseErrs>;

    fn number_of_blocks(&self) -> Option<u64> {
        self.count_rows(B::get_name())
    }

    fn number_of_records(&self) -> Option<u64> {
        self.count_rows(R::get_name())
    }

    /// 
    fn count_rows(&self, table_name: &str) -> Option<u64>;

    /// Checks if there is a table with name `table_name` present in the database
    fn table_exists(&self, table_name: &str) -> bool;

    /// Inserts the given `BlockBaseInsertable` item into its table in the database
    fn insert(&mut self, block: B) -> Result<Block<X>, BlockBaseErrs> {
        let begin = match self.count_rows(R::get_name()) {
            Some(v) => v,
            _=> return Err(BlockBaseErrs::NoSuchTable(R::get_name())),
        };

        let end = begin + block.size() - 1;
        


        todo!()
    }

    fn prev_block_hash(&self) -> Vec<u8>;
}
