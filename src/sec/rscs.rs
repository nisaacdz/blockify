use serde::{Deserialize, Serialize};

use super::{SigningError, VerificationError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    buffer: Box<[u8]>,
}

impl PrivateKey {
    pub fn new(buffer: Box<[u8]>) -> PrivateKey {
        Self { buffer }
    }

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
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }

    pub fn into_public_key(self) -> PublicKey {
        self.into()
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.algorithm
    }

    pub fn sign(&self, msg: &[u8]) -> Result<DigitalSignature, SigningError> {
        self.algorithm.sign(msg, self)
    }
}

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

use ring::signature::{
    EcdsaKeyPair, EcdsaSigningAlgorithm, Ed25519KeyPair, RsaEncoding, RsaKeyPair,
    UnparsedPublicKey, VerificationAlgorithm,
};

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
    let key =
        EcdsaKeyPair::from_private_key_and_public_key(algo, key.private_key(), key.public_key())?;
    let signature = key.sign(&rng, msg)?;
    let buffer = signature.as_ref().to_vec();
    Ok(buffer.into())
}

fn sign_ed25519(msg: &[u8], key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
    let key = Ed25519KeyPair::from_seed_and_public_key(key.private_key(), key.public_key())?;
    let signature = key.sign(msg).as_ref().to_vec();
    Ok(signature.into())
}

fn sign_rsa(
    msg: &[u8],
    key: &AuthKeyPair,
    padding: &'static dyn RsaEncoding,
) -> Result<DigitalSignature, SigningError> {
    let rng = ring::rand::SystemRandom::new();
    let private_key = RsaKeyPair::from_der(key.private_key())?;

    let mut signature_vec = vec![0u8; private_key.public_modulus_len()];
    private_key.sign(padding, &rng, &msg, &mut signature_vec)?;

    Ok(signature_vec.into())
}
