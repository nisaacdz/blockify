use std::error::Error;

use rand::{thread_rng, Rng};

use sha2::{Digest, Sha256};

pub mod merkle;
mod plus;
mod temp;
pub use plus::*;

/// An error that can occur while signing a piece of message
#[derive(Debug, Clone, Copy)]
pub enum SigningError {
    KeyRejected,
    Unspecified,
    SerializationError,
}

impl std::fmt::Display for SigningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for SigningError {}

impl From<ring::error::KeyRejected> for SigningError {
    fn from(_: ring::error::KeyRejected) -> Self {
        SigningError::KeyRejected
    }
}

impl From<ring::error::Unspecified> for SigningError {
    fn from(_: ring::error::Unspecified) -> Self {
        SigningError::Unspecified
    }
}

/// An error that can occur while verifying a digital signature
#[derive(Debug, Clone, Copy)]
pub enum VerificationError {
    InvalidSignature,
    NoMatch,
    BadKey,
    Unspecified,
    SerializationError,
}

impl Error for VerificationError {}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<ring::error::Unspecified> for VerificationError {
    fn from(_: ring::error::Unspecified) -> Self {
        VerificationError::Unspecified
    }
}

/// Hashes the provided data using the SHA-256 algorithm and returns the computed hash.
///
/// # Arguments
///
/// * `data` - The data to be hashed. It must implement the `serde::Serialize` trait.
///
/// # Returns
///
/// The computed hash as a `Hash` type.
pub fn hash<T: Sized + serde::Serialize>(data: &T) -> Hash {
    // Serialize the input data into a binary format using the `bincode` crate.
    let bytes = bincode::serialize(data).unwrap();
    let buffer = hash_bytes(&bytes);
    buffer.into()
}

/// Hashes the given byte slice using the SHA-256 algorithm and returns the resulting hash as a byte vector.
///
/// # Arguments
///
/// * `bytes` - The binary data to be hashed.
///
/// # Returns
///
/// The computed hash as a `Vec<u8>`.
pub fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
    // Create a new instance of the SHA-256 hasher from the `sha2` crate.
    let mut hasher = Sha256::new();
    // Update the hash with the binary data.
    hasher.update(bytes);
    // Finalize the hash computation and store the result in `data`.
    let data = hasher.finalize();
    // Convert the `data` to a `Vec<u8>` for easier use.
    data.to_vec()
}

use crate::{
    block::UnchainedInstance,
    data::{Position, Timestamp},
    record::Record,
};
use serde::Serialize;

/// Hashes a block of records along with other parameters to compute the block's hash using the SHA-256 algorithm.
///
/// # Arguments
///
/// * `block` - The block of records to be hashed.
/// * `prev_hash` - The previous block's hash.
/// * `timestamp` - The timestamp associated with the block.
/// * `position` - The position of the block.
///
/// # Returns
///
/// The computed hash as a `Hash` type.
pub fn hash_block<R: Record + Serialize>(
    block: &UnchainedInstance<R>,
    prev_hash: &Hash,
    timestamp: &Timestamp,
    position: &Position,
) -> Hash {
    let records = bincode::serialize(block.records()).unwrap().into();
    let timestamp = bincode::serialize(timestamp).unwrap().into();
    let position = bincode::serialize(position).unwrap().into();
    let buffer = sha_from_x([
        prev_hash,
        &records,
        block.merkle_root(),
        &timestamp,
        &position,
    ]);
    buffer.into()
}

/// Generates a random SHA-256 hash.
///
/// # Returns
///
/// The randomly generated hash as a `Hash` type.
pub fn random_sha256() -> Hash {
    let mut bytes = vec![0; 32];
    thread_rng().fill(&mut bytes[..]);
    bytes.into()
}

/// Generates a random byte array of size `N`.
///
/// # Type Parameter
///
/// * `N` - The size of the byte array.
///
/// # Returns
///
/// The randomly generated byte array.
pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0; N];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

/// Generates a random byte vector of the specified length.
///
/// # Arguments
///
/// * `len` - The length of the byte vector to generate.
///
/// # Returns
///
/// The randomly generated byte vector.
pub fn random_bytes_vec(len: usize) -> Vec<u8> {
    let mut bytes = vec![0; len];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

/// Hashes the provided value using the SHA-256 algorithm and returns the computed hash.
///
/// # Arguments
///
/// * `value` - The value to be hashed.
///
/// # Returns
///
/// The computed hash as a `Hash` type.
pub fn sha<H: AsRef<[u8]>>(value: &H) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize().to_vec().into()
}

/// Computes the combined `SHA-256` hash of an array of values.
///
/// # Arguments
///
/// * `values` - An array of values to be hashed.
///
/// # Type Parameters
///
/// * `H` - The type of the values that can be hashed.
/// * `N` - The number of values in the array.
///
/// # Returns
///
/// The computed hash as a `Hash` type.
pub fn sha_from_x<H: AsRef<[u8]>, const N: usize>(values: [&H; N]) -> Hash {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value);
    }
    hasher.finalize().to_vec().into()
}

/// Verifies whether a given object's hash matches the provided hash value.
///
/// # Arguments
///
/// * `obj` - The object to be hashed and compared.
/// * `value` - The hash value to be compared against.
///
/// # Returns
///
/// `true` if the object's hash matches the provided hash value, otherwise `false`.
pub fn verify_hash<T: Sized + serde::Serialize>(obj: &T, value: &Hash) -> bool {
    value == &hash(obj)
}

/// Generates a new Ed25519 key pair using the `ed25519_dalek` crate and returns it as an `AuthKeyPair`.
///
/// The key pair consists of a private key and a corresponding public key, both serialized as byte vectors.
///
/// # Returns
///
/// An `AuthKeyPair` containing the generated key pair and the `KeyPairAlgorithm` used.

pub fn generate_ed25519_key_pair() -> AuthKeyPair {
    let mut rng = rand::thread_rng();

    // Serialize the private and public keys as byte vectors
    let keypair = ed25519_dalek::Keypair::generate(&mut rng);

    let public_key = keypair.public.as_ref().to_vec();
    let private_key = keypair.secret.as_ref().to_vec();

    // Returns the public and private keys as byte vectors
    AuthKeyPair::new(
        private_key.into_boxed_slice(),
        public_key.into_boxed_slice(),
        KeyPairAlgorithm::Ed25519,
    )
}

/// Verifies the Ed25519 digital signature for the given message using a public key.
///
/// # Arguments
///
/// * `msg` - The message that was signed.
/// * `signature` - The digital signature to be verified.
/// * `signer` - The public key of the signer.
///
/// # Returns
///
/// An `Ok(())` result if the signature is valid, otherwise returns a `VerificationError`.
///
/// # Errors
///
/// The function can return the following `VerificationError` values:
///
/// * `InvalidSignature` - If the provided signature is invalid or malformed.
/// * `NoMatch` - If the signature does not match the provided message.
/// * `BadKey` - If the provided public key is invalid or malformed.
pub fn verify_signature_ed25519(
    msg: &[u8],
    signature: DigitalSignature,
    signer: PublicKey,
) -> Result<(), VerificationError> {
    let dalek = ed25519_dalek::Signature::from_bytes(signature.buffer())
        .map_err(|_| VerificationError::InvalidSignature)?;
    match ed25519_dalek::PublicKey::from_bytes(signer.buffer()) {
        Ok(key) => match ed25519_dalek::Verifier::verify(&key, msg, &dalek) {
            Ok(_) => Ok(()),
            Err(_) => Err(VerificationError::NoMatch),
        },
        Err(_) => Err(VerificationError::BadKey),
    }
}

/// Signs a message using the provided authentication key pair and returns the digital signature.
///
/// # Arguments
///
/// * `msg` - The message to be signed.
/// * `key` - The authentication key pair used for signing.
///
/// # Returns
///
/// An `Ok` result containing the digital signature if the signing operation is successful,
/// otherwise returns a `SigningError`.
pub fn sign_msg(msg: &[u8], key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
    key.sign(msg)
}

/// Verifies the digital signature for the given message using the provided public key.
///
/// # Arguments
///
/// * `msg` - The message that was signed.
/// * `signature` - The digital signature to be verified.
/// * `signer` - The public key of the signer.
///
/// # Returns
///
/// An `Ok(())` result if the signature is valid, otherwise returns a `VerificationError`.
pub fn verify_signature(
    msg: &[u8],
    signature: &DigitalSignature,
    signer: &PublicKey,
) -> Result<(), VerificationError> {
    signer.verify(msg, signature)
}
