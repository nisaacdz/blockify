mod sqlite_block;
mod sqlite_chain;
mod generic;

pub use generic::*;
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

pub(crate) struct WrapperMut<T> {
    val: std::cell::UnsafeCell<T>,
}

impl<T> WrapperMut<T> {
    pub fn new(value: T) -> Self {
        Self {
            val: std::cell::UnsafeCell::new(value),
        }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { self.val.get().as_mut().unwrap() }
    }
}
