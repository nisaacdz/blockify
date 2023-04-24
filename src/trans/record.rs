use serde::{Deserialize, Serialize};

use crate::{
    refs::MetaData,
    sec::{self, rscs::*, VerificationError},
};

/// # Record
///
/// This trait defines the structure and properties of any data or information that needs to be securely and transparently stored on the blockchain, referred to as a `Record` object.
///
/// `Transparently` here means that the data should be provable and verifiable, although it may not necessarily be viewable by uninvolved parties.
///
/// `Provable` means that the authenticity of the data can be demonstrated or confirmed, while `verifiable` means that the occurrence of the record can be demonstrated or confirmed.
///
/// # What can be a Record?
///
/// Examples of `Records` include transaction data, metadata, votes, smart contract states, and any other type of information that needs to be recorded and verified on a blockchain.
///
/// # Usage
///
/// To use this blockchain library, users must implement the `Record` trait on their type. This trait includes methods for serializing and deserializing the data, as well as for verifying the integrity and authenticity of the record on the blockchain.
///
/// By implementing the `Record` trait on their type, users can ensure that their data is securely and transparently recorded on the blockchain, with all the benefits of decentralization, transparency, and immutability that blockchain technology provides.
///
/// # Methods
///
/// This trait includes the following methods:
///
/// - `record`: Signs the record with the given public and private keys, and generates a `SignedRecord`.
/// - `sign`: Signs the record with the given private key and returns a digital signature.
/// - `hash`: Computes and returns the hash of the record.
/// - `metadata`: Returns metadata associated with the record, if any.
/// - `verify`: Returns `Ok()` when signature verification succeeds or a `VerificationError` if it fails.
///
///
/// # Examples
/// ```
/// use blockify::{sec, trans::record::Record};
/// use serde::{Serialize, Deserialize};
/// use record_derive::Record;
///
/// #[derive(Clone, Serialize, Deserialize, Record)]
/// struct Vote {
///     session: i32,
///     choice: i32,
/// }
///
/// let keypair = sec::generate_ed25519_key_pair();
/// let my_record = Vote { session: 0, choice: 2 };
///
/// let signature = my_record.sign(&keypair).unwrap();
///
/// assert!(my_record.verify(signature, keypair.into_public_key()).is_ok())
/// ```
///
pub trait Record: Serialize + for<'a> Deserialize<'a> {
    fn record(self, keypair: AuthKeyPair) -> Result<SignedRecord<Self>, RecordError> {
        let signature = self.sign(&keypair)?;
        let hash = self.hash();
        let metadata = self.metadata();
        Ok(SignedRecord::new(
            self,
            signature,
            keypair.into_public_key(),
            hash,
            metadata,
        ))
    }

    fn sign(&self, key: &AuthKeyPair) -> Result<DigitalSignature, RecordError> {
        let msg = bincode::serialize(self).map_err(|_| RecordError::serialization_error())?;
        let signature = sec::sign_msg(&msg, key).map_err(|_| RecordError::default())?;
        Ok(signature)
    }

    fn verify(&self, signature: DigitalSignature, key: PublicKey) -> Result<(), VerificationError> {
        let msg = bincode::serialize(self).map_err(|_| VerificationError::SerializationError)?;
        key.verify(&msg, &signature)
    }

    fn hash(&self) -> Hash {
        sec::hash(self)
    }

    fn metadata(&self) -> MetaData {
        MetaData::empty()
    }
}

#[derive(Debug, Clone, Default)]
pub enum ErrorCode {
    #[default]
    CouldNotSerialize,
}

#[derive(Debug, Clone, Default)]
pub struct RecordError {
    code: ErrorCode,
    src: &'static str,
}

impl RecordError {
    const fn new(code: ErrorCode, src: &'static str) -> RecordError {
        RecordError { code, src }
    }
    const fn serialization_error() -> RecordError {
        RecordError::new(ErrorCode::CouldNotSerialize, "unknown")
    }
}

const RECORDS: [&'static str; 3] = ["Record", "Signature", "Signer"];
const NAME: &'static str = "Records";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignedRecord<R> {
    key: PublicKey,
    signature: DigitalSignature,
    hash: Hash,
    record: R,
    metadata: MetaData,
}

impl<R: Record> SignedRecord<R> {
    pub fn new(
        record: R,
        signature: DigitalSignature,
        key: PublicKey,
        hash: Hash,
        metadata: MetaData,
    ) -> Self {
        Self {
            record,
            signature,
            hash,
            key,
            metadata,
        }
    }

    pub fn signature(&self) -> &[u8] {
        &self.signature.buffer
    }

    pub fn record(&self) -> &R {
        &self.record
    }

    pub fn signer(&self) -> &PublicKey {
        &self.key
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.key.algorithm()
    }

    pub fn verify_signature(&self) -> Result<(), VerificationError> {
        let msg =
            bincode::serialize(self.record()).map_err(|_| VerificationError::SerializationError)?;
        sec::verify_signature(&msg, &self.signature, &self.key)
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn metadata(&self) -> MetaData {
        self.record().metadata()
    }
}
