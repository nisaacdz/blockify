use rand::{thread_rng, Rng};
use serde::Serialize;
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

use crate::{
    errs::*,
    refs::{MetaData, Range, TimeStamp},
    trans::{
        algos::KeyPairAlgorithm,
        blocks::BlockBuilder,
        record::{Record, SignedRecord},
    },
};
pub mod errs;
pub mod merkle;

type Hxsh = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

pub fn hash<T: Sized + Serialize>(data: &T) -> Vec<u8> {
    // Serialize the input data into a binary format using the `bincode` crate.
    let bytes = bincode::serialize(data).unwrap();
    hash_bytes(&bytes)
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
    block: &BlockBuilder<R>,
    prev_hash: &[u8],
    metadata: (&TimeStamp, &Range, &u64),
) -> Vec<u8> {
    let records = bincode::serialize(block.records()).unwrap();
    let metabytes = bincode::serialize(&metadata).unwrap();
    sha_from_4(prev_hash, &records, block.merkle_root(), &metabytes)
}

pub fn random_sha256() -> Vec<u8> {
    let mut bytes = vec![0; 32];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

pub fn random_bytes5() -> [u8; 5] {
    let mut bytes = [0; 5];
    thread_rng().fill(&mut bytes);
    bytes
}

pub fn random_bytes(len: usize) -> [u8] {
    let mut bytes = vec![0; 5];
    thread_rng().fill(&mut bytes);
    bytes
}

pub fn quick_id(len: i32) -> String {
    hex::encode(random_bytes(len))
}

pub fn sha_from_2(data: &[u8], data1: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.finalize().to_vec()
}

pub fn sha_from_3(data: &[u8], data1: &[u8], data2: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.finalize().to_vec()
}

pub fn sha_from_4(data: &[u8], data1: &[u8], data2: &[u8], data3: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.finalize().to_vec()
}

pub fn sha_from_5(data: &[u8], data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.update(data4);
    hasher.finalize().to_vec()
}

pub fn validate<T: Sized + Serialize>(obj: &T, value: &[u8]) -> bool {
    value == hash(obj)
}

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

pub struct AuthKeyPair(Vec<u8>, Vec<u8>);

impl AuthKeyPair {
    pub fn public_key(&self) -> &[u8] {
        &self.0
    }

    pub fn private_key(&self) -> &[u8] {
        &self.1
    }
}

pub fn generate_key_pair() -> AuthKeyPair {
    // Generate a new key pair
    let mut c = rand::rngs::OsRng;
    let keypair = Keypair::generate(&mut c);

    // Serialize the private and public keys as byte vectors
    let private_key = keypair.secret.to_bytes().to_vec();
    let public_key = keypair.public.to_bytes().to_vec();

    // Returns the public and private keys as byte vectors
    AuthKeyPair(public_key, private_key)
}

pub fn sign_msg(msg: &[u8], key: &[u8]) -> Result<Vec<u8>, Failure> {
    // Parse the private key
    match SecretKey::from_bytes(key) {
        Ok(secret) => {
            // Create a signature for the message
            let keypair = Keypair {
                public: PublicKey::from(&secret),
                secret,
            };
            let signature = keypair.sign(msg);
            Ok(signature.as_ref().to_vec())
        }
        Err(_) => Err(Failure::SigningFailure(
            errs::SigningFailures::InvalidPrivateKey,
        )),
    }
}

pub fn verify_signature_ed25519(
    msg: &[u8],
    signature: &[u8],
    signer: &[u8],
) -> Result<bool, GenErrs> {
    let dalek = Signature::from_bytes(signature).map_err(|_| GenErrs::InvalidSignature)?;
    match PublicKey::from_bytes(signer) {
        Ok(key) => match key.verify(msg, &dalek) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        },
        Err(_) => Err(GenErrs::InvalidPublicKey),
    }
}

use ring::signature::VerificationAlgorithm;

use self::errs::Failure;

pub fn verify_signature(
    msg: &[u8],
    signature: &[u8],
    signer: &[u8],
    algo: KeyPairAlgorithm,
) -> Result<(), ring::error::Unspecified> {
    let msg = untrusted::Input::from(msg);
    let public_key = untrusted::Input::from(signer);
    let signature = untrusted::Input::from(signature);

    match algo {
        KeyPairAlgorithm::ED25519 => ring::signature::ED25519.verify(public_key, msg, signature),
    }
}

pub fn sign<R: Record>(
    record: &R,
    public_key: &[u8],
    private_key: &[u8],
    algorithm: KeyPairAlgorithm,
    metadata: MetaData,
) -> Result<SignedRecord<R>, errs::Failure> {
    let msg = bincode::serialize(record).unwrap();
    let signature = sign_msg(&msg, private_key)?;
    let hash = hash_bytes(&msg);

    Ok(SignedRecord::new(
        record.clone(),
        signature,
        algorithm,
        public_key,
        hash,
        metadata,
    ))
}
