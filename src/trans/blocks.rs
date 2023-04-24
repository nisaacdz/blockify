use crate::{
    refs::{Range, TimeStamp},
    sec::merkle::MerkleTree,
};

use super::record::{Record, SignedRecord};
use crate::sec::rscs::*;

const COLUMNS: [&'static str; 5] = ["Hash", "Previous", "Merkle", "Range", "TimeStamp"];

const TITLE: &'static str = "Blockchain";

pub struct BlockError {}

pub struct Block {
    nonce: u64,
    position: u64,
    time_stamp: TimeStamp,
    hash: Hash,
    prev_hash: Hash,
    merkle_root: Hash,
    records_range: Range,
}

impl Block {
    pub fn new(
        nonce: u64,
        position: u64,
        time_stamp: TimeStamp,
        hash: Hash,
        prev_hash: Hash,
        merkle_root: Hash,
        range: Range,
    ) -> Self {
        Self {
            nonce,
            position,
            time_stamp,
            hash,
            prev_hash,
            merkle_root,
            records_range: range,
        }
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn prev_block_hash(&self) -> &Hash {
        &self.prev_hash
    }

    pub fn merkle_root(&self) -> &Hash {
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

    pub fn records<R: Record>(&self) -> Result<Vec<SignedRecord<R>>, BlockError> {
        todo!()
    }
}

/// Nodes may keep instances of block Copy in their local chains
///
/// BlockCopy consists of the original block and other metadata
///

pub struct BlockCopy {
    block: Block,
    local_position: u64,
}

impl BlockCopy {
    pub fn original_block(&self) -> &Block {
        &self.block
    }

    pub fn local_position(&self) -> u64 {
        self.local_position
    }
}

pub struct BlockBuilder<R: Record> {
    nonce: u64,
    records: Vec<SignedRecord<R>>,
    merkle: MerkleTree,
    merkle_root: Hash,
}

impl<R: Record> BlockBuilder<R> {
    pub fn merkle_root(&self) -> &Hash {
        &self.merkle_root
    }

    pub fn push(&mut self, item: SignedRecord<R>) -> Result<(), BlockError> {
        let hash = item.hash();
        self.merkle.push(hash);
        self.records.push(item);
        Ok(())
    }

    pub fn records(&self) -> &Vec<SignedRecord<R>> {
        &self.records
    }
}
