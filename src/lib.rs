//! # Blockify
//! 
//! A Rust blockchain library that provides the building blocks for creating a full-
//! fledged blockchain application or any application that needs certain blockchain features,allowing you to focus on the higher-
//! level features of your application without worrying about the low-level details 
//! of `block validation`, `data serialization`, and `cryptographic operations`.
//! 
//! 
//! This library provides various features and functionalities including:
//! 
//! - hashing
//! - signing
//! - signature verification
//! - creating a `SignedRecord` - A wrapper around any type that can be stored on a blockchain. It contains information like `hash`, `digital signature`, `public_key` of the signer of the transaction etc.
//! - building a block
//! - appending a block to a blockchain
//! - Managing `Units` or `Currency`
//! - Creating and deploying smart contracts
//! - Creating and managing consensus mechanisms
//! - Dealing with `Records`, `Blocks`, and `Chains`.

#![feature(generic_const_exprs)]

pub mod data;
pub mod io;
pub mod node;

#[cfg(feature = "crypto")]
mod crypto;

#[cfg(feature = "crypto")]
pub use crypto::*;

#[cfg(feature = "record")]
mod trans;

#[cfg(feature = "record")]
pub use trans::*;

pub mod verification;


mod essentials;

pub(crate) use essentials::*;