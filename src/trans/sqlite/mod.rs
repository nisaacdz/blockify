mod sqlite_block;
mod sqlite_chain;

pub use sqlite_block::*;
pub use sqlite_chain::*;

use crate::{
    data::{Nonce, Position, Timestamp},
    Hash,
};

pub struct TempInstance {
    pub nonce: Nonce,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: Timestamp,
    pub position: Position,
}

impl TempInstance {
    fn new(
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
            hash,
            prev_hash,
            merkle_root,
            timestamp,
        }
    }
}
