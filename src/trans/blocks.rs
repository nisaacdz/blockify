use serde::{Deserialize, Serialize};

use crate::{
    crypto::*,
    data::{BlockRange, TimeStamp},
};

use super::{
    image::BlockImage,
    record::{Record, SignedRecord},
};

pub trait Block<X> {
    type RecordType: AsRef<SignedRecord<X>>;
    fn records(&self) -> Result<Vec<Self::RecordType>, BlockError>;
    fn image<T: BlockImage<X>>(&self) -> Result<T, BlockError>;
    fn hash(&self) -> Hash;
    fn merkle_root(&self) -> Hash;
    fn nonce(&self) -> u64;
}

pub struct BlockError {}

pub struct ChainedInstance {
    nonce: u64,
    position: u64,
    time_stamp: TimeStamp,
    hash: Hash,
    prev_hash: Hash,
    merkle_root: Hash,
    records_range: BlockRange,
}

impl ChainedInstance {
    pub fn new(
        nonce: u64,
        position: u64,
        time_stamp: TimeStamp,
        hash: Hash,
        prev_hash: Hash,
        merkle_root: Hash,
        range: BlockRange,
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

    pub fn prev_hash(&self) -> &Hash {
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

    pub fn records_range(&self) -> BlockRange {
        self.records_range
    }

    pub fn records<R: Record>(&self) -> Result<Vec<SignedRecord<R>>, BlockError> {
        unimplemented!()
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Hash)]
pub struct UnchainedInstance<R> {
    records: Vec<SignedRecord<R>>,
    merkle: merkle::MerkleTree,
    merkle_root: Hash,
}

impl<R: Record> UnchainedInstance<R> {
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
