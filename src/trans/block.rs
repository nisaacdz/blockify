use serde::{Deserialize, Serialize};

use crate::{
    crypto::*,
    data::{BlockRange, MetaData, TimeStamp},
    io::{DataBaseError, SerdeError},
};

use super::{
    chain::ChainError,
    record::{Record, SignedRecord},
};

pub trait Block<X> {
    fn records(&self) -> Result<Box<[SignedRecord<X>]>, BlockError>;
    fn hash(&self) -> Result<Hash, BlockError>;
    fn merkle_root(&self) -> Result<Hash, BlockError>;
    fn validate(&self, chained: &ChainedInstance) -> Result<bool, BlockError> {
        let res = (self.nonce()?, &self.hash()?, &self.merkle_root()?)
            == (chained.nonce(), chained.hash(), chained.merkle_root());
        Ok(res)
    }
    fn nonce(&self) -> Result<u64, BlockError>;
}

pub enum BlockError {
    SerdeError(SerdeError),
    DataBaseError(DataBaseError),
    Unspecified,
}

impl From<ChainError> for BlockError {
    fn from(value: ChainError) -> Self {
        match value {
            ChainError::SerdeError(v) => BlockError::SerdeError(v),
            ChainError::DataBaseError(u) => BlockError::DataBaseError(u),
            ChainError::Unspecified => BlockError::Unspecified,
        }
    }
}

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

    pub fn records<R: Record, B: Block<R>>(
        &self,
        block: &B,
    ) -> Result<Box<[SignedRecord<R>]>, BlockError> {
        let res = block.records()?;
        Ok(res)
    }
}
/// Represents an unchained instance of a block. While a block is being assembled,
/// it is called an UnchainedInstance. It contains a collection of signed records,
/// a Merkle tree, and the root hash of the Merkle tree.
#[derive(Serialize, Debug, Deserialize, Clone, Hash)]
pub struct UnchainedInstance<R> {
    records: Vec<SignedRecord<R>>,
    merkle: merkle::MerkleTree,
    metadata: MetaData,
}

impl<R: Record> UnchainedInstance<R> {
    pub fn merkle_root(&self) -> &Hash {
        &self.merkle.merkle_root()
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
