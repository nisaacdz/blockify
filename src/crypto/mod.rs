use std::error::Error;

use rand::{thread_rng, Rng};

use sha2::{Digest, Sha256};

pub mod merkle;

/// An error that can occur while signing a piece of message
#[derive(Debug, Clone, Copy)]
pub enum SigningError {
    KeyRejected,
    Unspecified,
    SerdeError(SerdeError),
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
    SerdeError(SerdeError),
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
    record::Record, io::SerdeError,
};
use serde::{Deserialize, Serialize};

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
    match ed25519_dalek::PublicKey::from_bytes(signer.as_bytes()) {
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
/// * `key` - The cryptographic key pair to be used
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

/// Serialize the given value into bytes.
/// 
/// Internally uses `bincode::serialize`
/// 
/// # Trait Bound
/// - `serde::Serialize`
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, SerdeError> {
    bincode::serialize(value).map_err(|_| SerdeError::SerializationError)
}

/// A `PrivateKey` is the secret component of an AuthKeyPair
/// TODO
/// Must fill comments here
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    bytes: Box<[u8]>,
}

impl PrivateKey {
    /// Creates a new `PrivateKey` from a boxed slice
    pub fn new(bytes: Box<[u8]>) -> PrivateKey {
        Self { bytes }
    }

    /// Returns a slice of the bytes of this key
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(bytes: Vec<u8>) -> Self {
        PrivateKey {
            bytes: bytes.into_boxed_slice(),
        }
    }
}
/// A `PublicKey` is a cryptographic key that can be used to verify digital signatures that are signed with the equivalent `AuthKeyPair`

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey {
    bytes: Box<[u8]>,
    algorithm: KeyPairAlgorithm,
}

impl PublicKey {
    pub fn new(bytes: Box<[u8]>, algorithm: KeyPairAlgorithm) -> PublicKey {
        Self { bytes, algorithm }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.algorithm
    }

    pub fn verify(
        &self,
        msg: &[u8],
        signature: &DigitalSignature,
    ) -> Result<(), VerificationError> {
        self.algorithm.verify(msg, signature, self.as_bytes())
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<PublicKey> for String {
    fn from(value: PublicKey) -> Self {
        value.to_hex()
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<AuthKeyPair> for PublicKey {
    fn from(value: AuthKeyPair) -> Self {
        let AuthKeyPair {
            private_key: _,
            public_key,
            algorithm,
        } = value;
        Self {
            bytes: public_key,
            algorithm,
        }
    }
}

/// An `AuthKeyPair` is a cryptographic key pair that can be used for digital signing and verification.
///
#[derive(Debug, Clone)]
pub struct AuthKeyPair {
    private_key: Box<[u8]>,
    public_key: Box<[u8]>,
    algorithm: KeyPairAlgorithm,
}

impl AuthKeyPair {
    pub fn new(
        private_key: Box<[u8]>,
        public_key: Box<[u8]>,
        algorithm: KeyPairAlgorithm,
    ) -> AuthKeyPair {
        AuthKeyPair {
            private_key,
            public_key,
            algorithm,
        }
    }
    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    }

    pub fn private_key_bytes(&self) -> &[u8] {
        &self.private_key
    }

    pub fn into_public_key(self) -> PublicKey {
        self.into()
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.algorithm
    }

    /// Uses this `AuthKeyPair` to sign the given bytes returning a digital signature
    pub fn sign(&self, msg: &[u8]) -> Result<DigitalSignature, SigningError> {
        self.algorithm.sign(msg, self)
    }
}

/// A `Hash` is the result of hashing a value.
///
/// A typical hash produces bytes as the output.
/// Since the exact size of the buffer varies depending on the hashing algorithm used,
/// the underlying buffer stored is a slice of bytes instead of `[u8; 32]` or `[u8; 48]` or any other.
///
/// This means that the slice of bytes itself will be stored on the heap.
///
/// This can lead to `very little runtime overhead` in some cases but it provides a robust structure for handling any kind of Hashes.
///

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash {
    bytes: Box<[u8]>,
}

impl Hash {
    pub fn new(bytes: Box<[u8]>) -> Hash {
        Hash { bytes }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
}

impl Default for Hash {
    fn default() -> Self {
        let bytes = vec![0; 32];
        Hash::new(bytes.into_boxed_slice())
    }
}

impl From<Hash> for String {
    fn from(data: Hash) -> Self {
        hex::encode(data)
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl std::ops::Deref for Hash {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<Vec<u8>> for Hash {
    fn from(value: Vec<u8>) -> Hash {
        Hash::new(value.into_boxed_slice())
    }
}

/// A `DigitalSignature` is the output of signing a piece of data with an `AuthKeyPair`.
/// 
/// It is used to verify the authenticity of the data signed.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalSignature {
    pub(crate) buffer: Box<[u8]>,
}

impl DigitalSignature {
    pub fn new(buffer: Box<[u8]>) -> DigitalSignature {
        Self { buffer }
    }
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
    pub fn to_hex(&self) -> String {
        hex::encode(&self.buffer)
    }
}

impl From<Vec<u8>> for DigitalSignature {
    fn from(value: Vec<u8>) -> DigitalSignature {
        DigitalSignature::new(value.into_boxed_slice())
    }
}

impl From<DigitalSignature> for String {
    fn from(value: DigitalSignature) -> Self {
        value.to_hex()
    }
}

impl std::fmt::Display for DigitalSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl AsRef<[u8]> for DigitalSignature {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
    }
}

use ring::signature::{
    EcdsaKeyPair, EcdsaSigningAlgorithm, Ed25519KeyPair, RsaEncoding, RsaKeyPair,
    UnparsedPublicKey, VerificationAlgorithm,
};

/// An enum representing the different algorithms that can be used to generate key pairs.
///
/// Each variant of this enum represents the algorithm used for `signing` and `verifying` data
///
/// The following algorithms are supported:
///
/// * `Ed25519`: An elliptic curve digital signature algorithm.
/// * `Ecdsa256256Fixed`: An elliptic curve digital signature algorithm with a fixed curve.
/// * `RsaPKCS1256`: A Rivest–Shamir–Adleman algorithm with a 256-bit modulus.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyPairAlgorithm {
    Ed25519,
    Ecdsa256256Fixed,
    RsaPKCS1256,
}

impl std::fmt::Display for KeyPairAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl KeyPairAlgorithm {
    fn sign(self, msg: &[u8], key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
        match self {
            KeyPairAlgorithm::Ed25519 => sign_ed25519(msg, key),
            KeyPairAlgorithm::RsaPKCS1256 => {
                sign_rsa(msg, &key, &ring::signature::RSA_PKCS1_SHA256)
            }
            KeyPairAlgorithm::Ecdsa256256Fixed => {
                sign_ecdsa(msg, key, &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING)
            }
        }
    }

    pub fn verify(
        self,
        msg: &[u8],
        signature: &DigitalSignature,
        signer: &[u8],
    ) -> Result<(), VerificationError> {
        let algo: &dyn VerificationAlgorithm = match self {
            KeyPairAlgorithm::Ed25519 => &ring::signature::ED25519,
            KeyPairAlgorithm::RsaPKCS1256 => &ring::signature::RSA_PKCS1_2048_8192_SHA256,
            KeyPairAlgorithm::Ecdsa256256Fixed => &ring::signature::ECDSA_P256_SHA256_FIXED,
        };

        let key = UnparsedPublicKey::new(algo, signer);
        key.verify(msg, signature.buffer())?;
        Ok(())
    }
}

fn sign_ecdsa(
    msg: &[u8],
    key: &AuthKeyPair,
    algo: &'static EcdsaSigningAlgorithm,
) -> Result<DigitalSignature, SigningError> {
    let rng = ring::rand::SystemRandom::new();
    let key = EcdsaKeyPair::from_private_key_and_public_key(
        algo,
        key.private_key_bytes(),
        key.public_key_bytes(),
    )?;
    let signature = key.sign(&rng, msg)?;
    let buffer = signature.as_ref().to_vec();
    Ok(buffer.into())
}

fn sign_ed25519(msg: &[u8], key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
    let key =
        Ed25519KeyPair::from_seed_and_public_key(key.private_key_bytes(), key.public_key_bytes())?;
    let signature = key.sign(msg).as_ref().to_vec();
    Ok(signature.into())
}

fn sign_rsa(
    msg: &[u8],
    key: &AuthKeyPair,
    padding: &'static dyn RsaEncoding,
) -> Result<DigitalSignature, SigningError> {
    let rng = ring::rand::SystemRandom::new();
    let private_key = RsaKeyPair::from_der(key.private_key_bytes())?;

    let mut signature_vec = vec![0u8; private_key.public_modulus_len()];
    private_key.sign(padding, &rng, &msg, &mut signature_vec)?;

    Ok(signature_vec.into())
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    #[test]
    fn hash_test() {
        #[derive(Serialize)]
        struct DMS {
            audio: Option<String>,
            moving_pictures: Option<String>,
            metadata: String,
        }

        impl DMS {
            fn new(audio: Option<String>, mp: Option<String>, d: String) -> DMS {
                DMS {
                    audio,
                    moving_pictures: mp,
                    metadata: d,
                }
            }
        }

        impl Default for DMS {
            fn default() -> Self {
                DMS::new(None, None, String::new())
            }
        }

        let dms = DMS::default();
        let my_dms = DMS::new(None, Some("Harry Potter".into()), "".into());

        let dms_hash = crate::hash(&dms);
        let my_dms_hash = crate::hash(&my_dms);

        println!("{}", dms_hash.to_hex());
        println!("{}", my_dms_hash.to_hex());
    }
}
