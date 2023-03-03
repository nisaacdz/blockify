use std::{collections::HashSet, slice::Iter};

use crate::{errs::BlockBaseErrs, gen, Range, block::Block};

pub trait BlockBaseInsertable<R: RecordBaseInsertable> {
    fn get_name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn get_rows(&self) -> Iter<R>;

    fn len(&self) -> u64;
}

pub trait RecordBaseInsertable {
    fn get_name() -> &'static str;

    fn columns() -> &'static [&'static str];
}

pub trait BlockBase<R: RecordBaseInsertable> {
    /// Creates a new table with the given name, if table doesn't already exist.
    ///
    /// `Ok(())` value indicates success whiles `BlockBaseErrs` indicate different failures
    fn create_table(&mut self, table_name: &str, colums: &[&str]) -> Result<(), BlockBaseErrs>;

    /// Returns `None` if the table is not present in the database
    /// and `Some(number of rows)` if the table exists
    fn size_of_blocks_table<T: BlockBaseInsertable<R>>(&self) -> Option<u64> {
        if self.block_table_exists::<T>() {
            Some(self.len(T::get_name()))
        } else {
            None
        }
    }

    fn size_of_records_table(&self) -> Option<u64> {
        if self.record_table_exists() {
            Some(self.len(R::get_name()))
        }else{
            None
        }
    }

    /// Do not use, may fail
    fn len(&self, table_name: &str) -> u64;

    /// Checks if the given type has a table in the database
    fn block_table_exists<T: BlockBaseInsertable<R>>(&self) -> bool {
        self.get_tables().contains(T::get_name())
    }


    fn record_table_exists(&self) -> bool {
        self.get_tables().contains(R::get_name())
    }

    /// Returns a mutable reference to the set of table names in the database
    fn get_tables_mut(&mut self) -> &mut HashSet<String>;

    ///Inserts a new table for the given type it doesn't already exist
    ///
    ///It attempts to insert the table without checking if it is already present in `get_tables()`
    ///
    /// Use it if `get_tables()` might be misleading
    fn insert_table(&mut self, table_name: &str, columns: &[&str]) -> Result<(), BlockBaseErrs> {
        self.get_tables_mut().insert(table_name.to_owned());
        self.create_table(table_name, columns)
    }

    /// Returns an immutable reference to the tables in the database
    fn get_tables(&self) -> &HashSet<String>;

    /// Inserts the given Item into its table in the database
    fn insert<T: BlockBaseInsertable<R>>(
        &mut self,
        item: &T,
    ) -> Result<Block<R>, BlockBaseErrs> {
        if !self.block_table_exists::<T>() {
            self.insert_table(T::get_name(), T::columns())?;
        }
        if !self.record_table_exists() {
            self.insert_table(R::get_name(), R::columns())?;
        }
        let r = self.insert_record_table_exists(item)?;
        self.insert_block_table_exists()
    }
    /// Use with caution. Recommended `insert()`
    ///
    /// This inserts item into the table for T given that the table for T exists in the database
    fn insert_record_table_exists<T: BlockBaseInsertable<R>>(
        &self,
        item: &T,
    ) -> Result<(Range, Vec<u8>), BlockBaseErrs> {
        let begin = self.len(T::get_name());
        let phash = if begin == 0 {
            gen::default_hash()
        } else {
            self.prev_block_hash()
        };

        let end = begin + item.len() - 1;
        let mut records = item.get_rows();

        for _ in begin..=end {
            self.insert_row(records.next().unwrap())?
        }

        Ok((Range::new(begin, end), phash))
    }

    fn prev_block_hash(&self) -> Vec<u8>;
    /// Inserts an insertable object into the database
    fn insert_row<T: RecordBaseInsertable>(&self, record: &T) -> Result<(), BlockBaseErrs>;
}
