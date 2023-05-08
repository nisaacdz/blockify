use serde::{Deserialize, Serialize};

use super::{SigningError, VerificationError};

/// A `PrivateKey` is a cryptographic key that can be used to sign data.
///
/// # Fields
///
/// * `buffer`: The raw bytes of the private key.
///
/// # Methods
///
/// * `new()`: Creates a new `PrivateKey` from a slice of bytes.
/// * `buffer()`: Gets the raw bytes of the private key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    buffer: Box<[u8]>,
}

impl PrivateKey {
    /// Creates a new `PrivateKey` from a buffer of raw bytes.
    ///
    /// # Arguments
    /// * `buffer`: The buffer of raw bytes.
    ///
    /// # Returns
    /// A new `PrivateKey`.
    ///
    /// There is no checking of validity of the generated `PrivateKey`
    pub fn new(buffer: Box<[u8]>) -> PrivateKey {
        Self { buffer }
    }
    /// Returns the raw bytes of the private key.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(buffer: Vec<u8>) -> Self {
        PrivateKey {
            buffer: buffer.into_boxed_slice(),
        }
    }
}
/// A `PublicKey` is a cryptographic key that can be used to verify signatures.
///
/// It is composed of a buffer of raw bytes and an algorithm.
///
/// # Fields
///
/// * `buffer`: The raw bytes of the public key.
/// * `algorithm`: The algorithm used to generate the public key.
///
/// # Methods
///
/// * `new()`: Creates a new `PublicKey` from a `Box<[u8]>` and a `KeyPairAlgorithm`.
/// * `buffer()`: Returns a slice of the public key bytes.
/// * `algorithm()`: Returns the algorithm used to generate the public key.
/// * `verify()`: Verifies a signature against the public key.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicKey {
    buffer: Box<[u8]>,
    algorithm: KeyPairAlgorithm,
}

impl PublicKey {
    pub fn new(buffer: Box<[u8]>, algorithm: KeyPairAlgorithm) -> PublicKey {
        Self { buffer, algorithm }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.algorithm
    }

    pub fn verify(
        &self,
        msg: &[u8],
        signature: &DigitalSignature,
    ) -> Result<(), VerificationError> {
        self.algorithm.verify(msg, signature, self.buffer())
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
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
            buffer: public_key,
            algorithm,
        }
    }
}

/// An `AuthKeyPair` is a cryptographic key pair that can be used for authentication.
///
/// It is composed of a private key, a public key, and an algorithm.
///
/// # Fields
///
/// * `private_key`: The private key.
/// * `public_key`: The public key.
/// * `algorithm`: The algorithm used to generate the key pair.
///
/// # Methods
///
/// * `new()`: Creates a new `AuthKeyPair` from a private key, a public key, and an algorithm.
/// * `private_key()`: Gets the private key.
/// * `public_key()`: Gets the public key.
/// * `algorithm()`: Gets the algorithm used to generate the key pair.

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

    /// Uses this `AuthKeyPair` to sign the given bytes producing a returning a digital signature
    pub fn sign(&self, msg: &[u8]) -> Result<DigitalSignature, SigningError> {
        self.algorithm.sign(msg, self)
    }
}

/// A `Hash` is the result of hashing a value.
///
/// It is composed of a buffer of raw bytes.
///
/// # Fields
///
/// * `buffer`: The raw bytes of the hash.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash {
    buffer: Box<[u8]>,
}

impl Hash {
    pub fn new(buffer: Box<[u8]>) -> Hash {
        Hash { buffer }
    }
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl From<Hash> for String {
    fn from(data: Hash) -> Self {
        hex::encode(data)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
    }
}

impl From<Vec<u8>> for Hash {
    fn from(value: Vec<u8>) -> Hash {
        Hash::new(value.into_boxed_slice())
    }
}

/// A `DigitalSignature` is a cryptographic signature that can be used to verify the authenticity of a message.
///
/// It is composed of a buffer of raw bytes.
///
/// # Fields
///
/// * `buffer`: The raw bytes of the signature.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
}

impl From<Vec<u8>> for DigitalSignature {
    fn from(value: Vec<u8>) -> DigitalSignature {
        DigitalSignature::new(value.into_boxed_slice())
    }
}

impl AsRef<[u8]> for DigitalSignature {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
    }
}

impl From<DigitalSignature> for String {
    fn from(data: DigitalSignature) -> Self {
        hex::encode(data)
    }
}

use ring::signature::{
    EcdsaKeyPair, EcdsaSigningAlgorithm, Ed25519KeyPair, RsaEncoding, RsaKeyPair,
    UnparsedPublicKey, VerificationAlgorithm,
};

/// An enum representing the different algorithms that can be used to generate key pairs.
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
