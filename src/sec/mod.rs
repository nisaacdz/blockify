use rand::{thread_rng, Rng};
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

use crate::{
    dat::{Range, TimeStamp},
    trans::{blocks::Block, record::Record},
};

pub mod merkle;
pub mod rscs;

use rscs::*;

type Hxsh = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

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
    // Finalize the hash computation and store the result in a `Hxsh` value.
    let data: Hxsh = hasher.finalize();
    // Convert the `Hxsh` value to a `Vec<u8>` for easier use.
    data.to_vec()
}

pub fn hash_block<R: Record>(
    block: &Block<R>,
    prev_hash: &Hash,
    metadata: (&TimeStamp, &Range, &u64),
) -> Hash {
    let records = bincode::serialize(block.records()).unwrap().into();
    let metabytes = bincode::serialize(&metadata).unwrap().into();
    let buffer = sha_from_4(prev_hash, &records, block.merkle_root(), &metabytes);
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

pub fn sha_from_2(data: &[u8], data1: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.finalize().to_vec().into()
}

pub fn sha_from_3(data: &Hash, data1: &Hash, data2: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.finalize().to_vec().into()
}

pub fn sha_from_4(data: &Hash, data1: &Hash, data2: &Hash, data3: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.finalize().to_vec().into()
}

pub fn sha_from_5(data: &Hash, data1: &Hash, data2: &Hash, data3: &Hash, data4: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.update(data4);
    hasher.finalize().to_vec().into()
}

pub fn validate<T: Sized + serde::Serialize>(obj: &T, value: &Hash) -> bool {
    value == &hash(obj)
}

pub fn generate_ed25519_key_pair() -> AuthKeyPair {
    // Generate a new key pair
    let mut c = rand::rngs::OsRng;
    let keypair = ed25519_dalek::Keypair::generate(&mut c);

    // Serialize the private and public keys as byte vectors
    let private_key = keypair.secret.to_bytes().to_vec();
    let public_key = keypair.public.to_bytes().to_vec();

    // Returns the public and private keys as byte vectors
    AuthKeyPair::new(
        private_key.into(),
        public_key.into(),
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
