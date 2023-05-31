//! # Blockify
//!
//! A Rust blockchain library that provides the building blocks for creating a full-
//! fledged blockchain application or any application that needs certain blockchain features,allowing you to focus on the higher-
//! level features of your application without worrying about the low-level details
//! of `block validation`, `data serialization`, and `cryptographic operations`.
//!
//!
//! This library provides concise API for:
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

pub mod data;
pub mod io;

pub mod node;

pub mod crypto;

pub use crypto::*;

pub mod trans;

pub use trans::*;

pub mod blockchain;

pub use blockchain::*;
