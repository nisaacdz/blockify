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
    /// Creates a new `PrivateKey` from a boxed slice
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
/// A `PublicKey` is a cryptographic key that can be used to verify digital signatures that are signed with the equivalent `AuthKeyPair` or `PrivateKey`.
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

/// An `AuthKeyPair` is a cryptographic key pair that can be used for digital signing and verification.
///
#[derive(Debug, Clone)]
pub struct AuthKeyPair {
    private_key: Box<[u8]>,
    public_key: Box<[u8]>,
    algorithm: KeyPairAlgorithm,
}

impl AuthKeyPair {
    pub fn new(private_key: Box<[u8]>, public_key: Box<[u8]>, algorithm: KeyPairAlgorithm) -> AuthKeyPair {
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

    pub fn to_hex(&self) -> String {
        hex::encode(&self.buffer)
    }
}

impl Default for Hash {
    fn default() -> Self {
        let buffer = vec![0; 32];
        Hash::new(buffer.into_boxed_slice())
    }
}

impl Into<[u8; 32]> for Hash {
    fn into(self) -> [u8; 32] {
        let mut buffer = [0; 32];
        for i in 0..usize::min(32, self.buffer.len()) {
            buffer[i] = self.buffer[i];
        }
        buffer
    }
}

impl From<Hash> for String {
    fn from(data: Hash) -> Self {
        hex::encode(data)
    }
}

impl std::ops::Deref for Hash {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.buffer()
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

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Hash {
            buffer: value.into(),
        }
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

        let dms_hash = blockify::hash(&dms);
        let my_dms_hash = blockify::hash(&my_dms);

        println!("{}", dms_hash.to_hex());
        println!("{}", my_dms_hash.to_hex());
    }
}
