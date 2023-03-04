use std::{marker::PhantomData, slice::Iter};

use crate::{
    io::BlockBaseInsertable,
    record::{Record, SignedRecord},
    Range, TimeStamp,
};

use self::merkle::MerkleTree;

pub mod chain;
pub mod merkle;

const COLUMNS: [&'static str; 5] = ["Hash", "Previous", "Merkle", "Range", "TimeStamp"];

const TITLE: &'static str = "Blockchain";

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
    pub fn new(
        nonce: u64,
        position: u64,
        time_stamp: TimeStamp,
        hash: Vec<u8>,
        prev_block_hash: Vec<u8>,
        merkle_root: Vec<u8>,
        range: Range,
    ) -> Self {
        Self {
            nonce,
            position,
            time_stamp,
            hash,
            prev_block_hash,
            merkle_root,
            records_range: range,
            phantom_data: PhantomData,
        }
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

    pub fn records_range(&self) -> Range {
        self.records_range
    }
}

/// Nodes may keep instances of block Copy in their local chains
///
/// BlockCopy consists of the original block and other metadata
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

pub struct BlockBuilder<R: Record> {
    nonce: u64,
    records: Vec<SignedRecord<R>>,
    merkle: MerkleTree,
}

impl<R: Record> BlockBuilder<R> {
    pub fn merkle_root(&self) -> &[u8] {
        &self.merkle.merkle_root()
    }

    pub fn push(&mut self, _item: SignedRecord<R>) -> bool {
        todo!()
    }
}

impl<R: Record> BlockBaseInsertable<SignedRecord<R>, R> for BlockBuilder<R> {
    fn name() -> &'static str {
        &TITLE
    }

    fn columns() -> &'static [&'static str] {
        &COLUMNS
    }

    fn size(&self) -> u64 {
        self.records.len() as u64
    }

    fn hash(&self) -> &[u8] {
        self.merkle_root()
    }

    fn generate(
        &self,
        hash: Vec<u8>,
        prev_hash: Vec<u8>,
        time_stamp: TimeStamp,
        range: Range,
        position: u64,
    ) -> Block<R> {
        Block::new(
            self.nonce,
            position,
            time_stamp,
            hash,
            prev_hash,
            self.merkle_root().to_vec(),
            range,
        )
    }

    fn records(&self) -> Iter<SignedRecord<R>> {
        self.records.iter()
    }

    fn insertion(
        &self,
        hash: &[u8],
        prev: &[u8],
        range: Range,
        timestamp: TimeStamp,
    ) -> Vec<String> {
        vec![
            format! {"{:?}", hash},
            format! {"{:?}", prev},
            format! {"{:?}", self.merkle_root()},
            serde_json::to_string(&range).unwrap(),
            serde_json::to_string(&timestamp).unwrap(),
        ]
    }
}
