use serde::{Deserialize, Serialize};

use crate::{
    crypto::*,
    data::{Metadata, Nonce, Position, Timestamp},
    io::{DataBaseError, SerdeError},
    merkle::MerkleTree,
};

use super::{
    chain::ChainError,
    record::{Record, SignedRecord},
};

/// A block is a unit of data in a blockchain. It contains a number of records, a
/// previous hash, a position, a hash, a merkle root, a timestamp, and a nonce.
///
/// This `Block` trait provides methods for accessing these properties.
pub trait Block {
    /// The type of record that is stored in this block.
    type RecordType: Record;

    /// Returns a reference to the records in this block.
    fn records(&self) -> Result<Box<[SignedRecord<Self::RecordType>]>, BlockError>;

    /// Returns the previous hash of this block.
    fn prev_hash(&self) -> Result<Hash, BlockError>;

    /// Returns the position of this block in the blockchain.
    fn position(&self) -> Result<Position, BlockError>;

    /// Returns the hash of this block.
    fn hash(&self) -> Result<Hash, BlockError>;

    /// Returns the merkle root of this block.
    fn merkle_root(&self) -> Result<Hash, BlockError>;

    /// Validates this block against the provided chained instance.
    ///
    /// Returns an error if the block is not valid.
    fn validate(&self, chained: &ChainedInstance) -> Result<(), BlockError> {
        let ChainedInstance {
            nonce,
            position,
            timestamp,
            hash,
            prev_hash,
            merkle_root,
        } = chained;

        if self.nonce()? != *nonce {
            return Err(BlockError::NotValid(BlockData::Nonce));
        }
        if self.position()? != *position {
            return Err(BlockError::NotValid(BlockData::Position));
        }

        if self.timestamp()? != *timestamp {
            return Err(BlockError::NotValid(BlockData::Timestamp));
        }

        if &self.hash()? != hash {
            return Err(BlockError::NotValid(BlockData::Hash));
        }

        if &self.prev_hash()? != prev_hash {
            return Err(BlockError::NotValid(BlockData::PrevHash));
        }

        if &self.merkle_root()? != merkle_root {
            return Err(BlockError::NotValid(BlockData::MerkleRoot));
        }

        Ok(())
    }

    /// Returns the timestamp of this block.
    fn timestamp(&self) -> Result<Timestamp, BlockError>;

    /// Returns the nonce of this block.
    fn nonce(&self) -> Result<Nonce, BlockError>;
}

/// An error that can occur when working with blocks.
#[derive(Debug, Clone)]
pub enum BlockError {
    /// An error that occurred while serializing or deserializing a block.
    SerdeError(SerdeError),

    /// An error that occurred while accessing the database.
    DataBaseError(DataBaseError),

    /// The block is not valid.
    NotValid(BlockData),

    /// An unspecified error occurred.
    Unspecified,
}

/// The data that is stored in a block.
#[derive(Debug, Clone)]
pub enum BlockData {
    /// The hash of the block.
    Hash,

    /// The previous hash of the block.
    PrevHash,

    /// The merkle root of the block.
    MerkleRoot,

    /// The timestamp of the block.
    Timestamp,

    /// The nonce of the block.
    Nonce,

    /// The position of the block in the blockchain.
    Position,
}

impl std::error::Error for BlockError {}

impl std::fmt::Display for BlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
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
    pub nonce: Nonce,
    pub position: Position,
    pub timestamp: Timestamp,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub merkle_root: Hash,
}

impl ChainedInstance {
    pub fn new(
        nonce: Nonce,
        position: Position,
        timestamp: Timestamp,
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

    pub fn timestamp(&self) -> Timestamp {
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
    metadata: Metadata,
    nonce: Nonce,
}

impl<R> UnchainedInstance<R> {
    pub fn new(metadata: Metadata, nonce: u64) -> Self {
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
