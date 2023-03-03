use std::marker::PhantomData;

use crate::{
    record::{Record, SignedRecord},
    TimeStamp, Range,
};

pub struct Block<R: Record> {
    nonce: u64,
    position: u64,
    time_stamp: TimeStamp,
    hash: Vec<u8>,
    prev_block_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    records_range: Range,
    phantom_data: PhantomData<R>,
}

impl<R: Record> Block<R> {
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

    pub fn records_range(&self) -> Range {
        self.records_range
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


struct BlockAssemblage<R: Record> {
    records: SignedRecord<R>,
    
}
