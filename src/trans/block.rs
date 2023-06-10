use serde::{Deserialize, Serialize};

use crate::{
    chain::Chain,
    crypto::*,
    data::{Metadata, Nonce, Position, Timestamp},
    error::{DataBaseError, SerdeError},
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
pub trait Block<R: Record> {
    /// Returns a reference to the records in this block.
    fn records(&self) -> Result<Box<[SignedRecord<R>]>, BlockError>;

    /// Returns the previous hash of this block.
    fn prev_hash(&self) -> Result<Hash, BlockError>;

    /// Returns the position of this block in the blockchain.
    fn position(&self) -> Result<Position, BlockError>;

    /// Returns the hash of this block.
    fn hash(&self) -> Result<Hash, BlockError>;

    /// Returns the merkle root of this block.
    fn merkle_root(&self) -> Result<Hash, BlockError>;

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

impl From<Position> for PositionInstance {
    fn from(value: Position) -> Self {
        Self::new(value)
    }
}

impl PositionInstance {
    pub fn block<R: Record, C: Chain<R>>(self, chain: &C) -> Result<C::BlockType, ChainError> {
        chain.get(self)
    }
    pub fn into_inner(self) -> Position {
        let Self { pos } = self;
        pos
    }
}

/// A type that may be used as a `ChainedInstance` in any `Chain` that only needs the position of a block to locate a block
///
pub struct PositionInstance {
    pub pos: Position,
}

impl PositionInstance {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct LocalInstance<R> {
    pub records: Vec<SignedRecord<R>>,
    pub merkle: merkle::MerkleTree,
    pub metadata: Metadata,
    pub nonce: Nonce,
}

impl<R> LocalInstance<R> {
    pub fn new(metadata: Metadata, nonce: u64) -> Self {
        Self {
            records: vec![],
            merkle: MerkleTree::new(),
            metadata,
            nonce: nonce.into(),
        }
    }
}

impl<R> LocalInstance<R> {
    pub fn push(&mut self, item: SignedRecord<R>) {
        let hash = item.hash();
        self.merkle.push(hash);
        self.records.push(item);
    }

    pub fn get_records(&self) -> &Vec<SignedRecord<R>> {
        &self.records
    }

    pub fn get_merkle_root(&self) -> &Hash {
        self.merkle.root()
    }
}

pub trait UnchainedInstance<R> {
    fn append(&mut self, item: SignedRecord<R>) -> Result<(), BlockError>;
    fn nonce(&self) -> Result<Nonce, BlockError>;
    fn records(&self) -> Result<Vec<SignedRecord<R>>, BlockError>;
    fn merkle_root(&self) -> Result<Hash, BlockError>;
}

impl<R: Clone> UnchainedInstance<R> for LocalInstance<R> {
    fn append(&mut self, item: SignedRecord<R>) -> Result<(), BlockError> {
        self.records.push(item);
        Ok(())
    }

    fn nonce(&self) -> Result<Nonce, BlockError> {
        Ok(self.nonce)
    }

    fn records(&self) -> Result<Vec<SignedRecord<R>>, BlockError> {
        Ok(self.records.clone())
    }

    fn merkle_root(&self) -> Result<Hash, BlockError> {
        Ok(self.merkle.root().clone())
    }
}
