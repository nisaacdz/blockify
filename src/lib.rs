//! # Blockify
//!
//! A Rust blockchain library that provides the building blocks for creating a full-fledged blockchain application or platform, allowing you to focus on the higher-level features of your application without worrying about the low-level details of `block validation`, `data serialization`, `blockchain technology`, and `cryptographic operations`.
//!
//!
//! This library provides concise API for:
//!
//! - generating various `cryptographic keys`
//! - `serializing`, `hashing`, and `signing` of different kinds of data
//! - building and verifying `blocks`
//! - building `blockchains` and appending `blocks` to them
//! - creating and deploying `smart contracts`
//! - building and managing `consensus protocols`
//! - `merging of forked chains` based on consensus rules

pub mod data;
pub mod error;

pub mod node;

pub mod crypto;

pub use crypto::*;

pub mod trans;

pub use trans::*;

pub mod blockchain;

pub use blockchain::*;

#[macro_export]
macro_rules! impl_display_error {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl std::error::Error for $type {}
    };
}
