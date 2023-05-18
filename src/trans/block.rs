use serde::{Deserialize, Serialize};

use crate::{
    crypto::*,
    data::{MetaData, Nonce, Position, TimeStamp},
    io::{DataBaseError, SerdeError},
    merkle::MerkleTree,
};

use super::{
    chain::ChainError,
    record::{Record, SignedRecord},
};

pub trait Block {
    type RecordType: Record;
    fn records(&mut self) -> Result<Box<[SignedRecord<Self::RecordType>]>, BlockError>;
    fn hash(&mut self) -> Result<Hash, BlockError>;
    fn merkle_root(&mut self) -> Result<Hash, BlockError>;
    fn validate(&mut self, chained: &ChainedInstance) -> Result<bool, BlockError> {
        let res = (self.nonce()?, &self.hash()?, &self.merkle_root()?)
            == (chained.nonce(), chained.hash(), chained.merkle_root());
        Ok(res)
    }
    fn nonce(&mut self) -> Result<Nonce, BlockError>;
}

#[derive(Debug, Clone)]
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
            ChainError::AbsentValue => unimplemented!(),
        }
    }
}

pub struct ChainedInstance {
    nonce: Nonce,
    position: Position,
    timestamp: TimeStamp,
    hash: Hash,
    prev_hash: Hash,
    merkle_root: Hash,
}

impl ChainedInstance {
    pub fn new(
        nonce: Nonce,
        position: Position,
        timestamp: TimeStamp,
        hash: Hash,
        prev_hash: Hash,
        merkle_root: Hash,
    ) -> Self {
        Self {
            nonce,
            position,
            timestamp,
            hash,
            prev_hash,
            merkle_root,
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

    pub fn timestamp(&self) -> TimeStamp {
        self.timestamp
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn records<R: Record, B: Block<RecordType = R>>(
        &self,
        block: &mut B,
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
    nonce: Nonce,
}

impl<R> UnchainedInstance<R> {
    pub fn new(metadata: MetaData, nonce: u64) -> Self {
        Self {
            records: vec![],
            merkle: MerkleTree::new(),
            metadata,
            nonce: nonce.into(),
        }
    }
    pub fn merkle_root(&self) -> &Hash {
        &self.merkle.merkle_root()
    }

    pub fn records(&self) -> &Vec<SignedRecord<R>> {
        &self.records
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }
}

impl<R: Record> UnchainedInstance<R> {
    pub fn push(&mut self, item: SignedRecord<R>) {
        let hash = item.hash();
        self.merkle.push(hash);
        self.records.push(item);
    }
}
