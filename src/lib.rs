//! # Blockify
//! 
//! A Rust blockchain library that provides the building blocks for creating a full-
//! fledged blockchain application or platform,allowing you to focus on the higher-
//! level features of your application without worrying about the low-level details 
//! of `block validation`, `data serialization`, and `cryptographic operations`.
//! 
//! 
//! This library provides various features and functionalities including:
//! 
//! - hashing
//! - signing
//! - signature verification
//! - creating a `SignedRecord`
//! - building a block
//! - appending a block to a blockchain
//! - Managing `Units` or `Currency`
//! - Creating and deploying smart contracts
//! - Creating and managing consensus mechanisms
//! - Dealing with `Records`, `Blocks`, and `Chains`.
//! 


pub mod data;
pub mod io;
pub mod node;

#[cfg(feature = "crypto")]
mod crypto;

#[cfg(feature = "crypto")]
pub use crypto::*;

mod trans;
pub use trans::*;

pub mod ver;

mod fxns;
pub use fxns::*;
