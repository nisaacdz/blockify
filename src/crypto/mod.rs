use rand::{thread_rng, Rng};
#[cfg(feature = "blockchain")]
use serde::Serialize;

use sha2::{Digest, Sha256};

#[cfg(feature = "blockchain")]
use crate::data::{BlockRange, TimeStamp};

#[cfg(feature = "blockchain")]
use crate::trans::{blocks::UnchainedInstance, record::Record};

pub mod merkle;
mod plus;

pub use plus::*;

#[derive(Debug, Clone, Copy)]
pub enum SigningError {
    KeyRejected,
    Unspecified,
    SerializationError,
}

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

pub enum VerificationError {
    InvalidSignature,
    NoMatch,
    BadKeyPair,
    Unspecified,
    SerializationError,
}

impl From<ring::error::Unspecified> for VerificationError {
    fn from(_: ring::error::Unspecified) -> Self {
        VerificationError::Unspecified
    }
}

pub fn hash<T: Sized + serde::Serialize>(data: &T) -> Hash {
    // Serialize the input data into a binary format using the `bincode` crate.
    let bytes = bincode::serialize(data).unwrap();
    let buffer = hash_bytes(&bytes);
    buffer.into()
}

pub fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
    // Create a new instance of the SHA-256 hasher from the `sha2` crate.
    let mut hasher = Sha256::new();
    // Update the hash with the binary data.
    hasher.update(bytes);
    // Finalize the hash computation and store the result as `data`.
    let data = hasher.finalize();
    // Convert the `data` to a `Vec<u8>` for easier use.
    data.to_vec()
}

#[cfg(feature = "blockchain")]
pub fn hash_block<R: Record + Serialize>(
    block: &UnchainedInstance<R>,
    prev_hash: &Hash,
    metadata: (&TimeStamp, &BlockRange, &u64),
) -> Hash {
    let records = bincode::serialize(block.records()).unwrap().into();
    let metabytes = bincode::serialize(&metadata).unwrap().into();
    let buffer = sha_from_x([prev_hash, &records, block.merkle_root(), &metabytes]);
    buffer.into()
}

pub fn random_sha256() -> Hash {
    let mut bytes = vec![0; 32];
    thread_rng().fill(&mut bytes[..]);
    bytes.into()
}

pub fn random_bytes5() -> [u8; 5] {
    let mut bytes = [0; 5];
    thread_rng().fill(&mut bytes);
    bytes
}

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0; len];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

pub fn quick_id(len: usize) -> String {
    hex::encode(random_bytes(len))
}

pub fn sha<H: AsRef<[u8]>>(value: &H) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize().to_vec().into()
}

pub fn sha_from_x<H: AsRef<[u8]>, const N: usize>(values: [&H; N]) -> Hash {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value);
    }
    hasher.finalize().to_vec().into()
}

pub fn verify_hash<T: Sized + serde::Serialize>(obj: &T, value: &Hash) -> bool {
    value == &hash(obj)
}

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

pub fn sign_msg(msg: &[u8], key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
    key.sign(msg)
}

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
        Err(_) => Err(VerificationError::BadKeyPair),
    }
}

pub fn verify_signature(
    msg: &[u8],
    signature: &DigitalSignature,
    signer: &PublicKey,
) -> Result<(), VerificationError> {
    signer.verify(msg, signature)
}
