use std::slice::Iter;

use crate::{
    io::BlockBaseInsertable,
    record::{Record, SignedRecord},
    TimeStamp,
};

const BLOCKS: [&'static str; 5] = ["Hash", "Previous", "Merkle", "Range", "TimeStamp"];

const NAME: &'static str = "Blockchain";

pub struct Block<R: Record> {
    nonce: u64,
    position: u64,
    time_stamp: TimeStamp,
    hash: Vec<u8>,
    prev_block_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    records: Vec<SignedRecord<R>>,
}

impl<R: Record> Block<R> {
    pub fn get_records(&self) -> &Vec<SignedRecord<R>> {
        &self.records
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn prev_block_hash(&self) -> &[u8] {
        &self.prev_block_hash
    }

    pub fn merkle_root(&self) -> &[u8] {
        &self.merkle_root
    }

    pub fn time_stamp(&self) -> TimeStamp {
        self.time_stamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn position(&self) -> u64 {
        self.position
    }
}

impl<R: Record> BlockBaseInsertable<SignedRecord<R>> for Block<R> {
    fn get_name() -> &'static str {
        &NAME
    }

    fn columns() -> &'static [&'static str] {
        &BLOCKS
    }

    fn get_rows(&self) -> Iter<SignedRecord<R>> {
        self.records.iter()
    }

    fn len(&self) -> u64 {
        self.records.len() as u64
    }
}

/// Nodes may keep instances of block Copy in their local chains
/// BlockCopy consists of the original block plus other metadata
///
///

pub struct BlockCopy<R: Record> {
    block: Block<R>,
    local_position: u64,
}

impl<R: Record> BlockCopy<R> {
    pub fn original_block(&self) -> &Block<R> {
        &self.block
    }

    pub fn local_position(&self) -> u64 {
        self.local_position
    }
}
