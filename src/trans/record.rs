use serde::{Deserialize, Serialize};

use crate::{
    dat::MetaData,
    sec::{self, rscs::*, SigningError, VerificationError},
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
/// - `verify`: Returns `Ok()` if signature verification succeeds or a `VerificationError` if it fails.
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
/// assert!(my_record.verify(&signature, &keypair.into_public_key()).is_ok())
/// ```
///
pub trait Record: Serialize + for<'a> Deserialize<'a> {
    /// converts `self` into a `SignedRecord` instance by singing it with the provided key pair
    ///
    /// # Arguments
    /// `AuthKeyPair` - The Keypair for the signing.
    ///
    /// # Returns
    /// - `Ok()` -> A `SignedRecord<T>` instance.
    /// - `Err()` -> A `SigningError` instance.
    ///
    fn record(self, keypair: AuthKeyPair) -> Result<SignedRecord<Self>, SigningError> {
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

    fn sign(&self, key: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
        let msg = bincode::serialize(self).map_err(|_| SigningError::SerializationError)?;
        let signature = sec::sign_msg(&msg, key)?;
        Ok(signature)
    }

    fn verify(
        &self,
        signature: &DigitalSignature,
        key: &PublicKey,
    ) -> Result<(), VerificationError> {
        let msg = bincode::serialize(self).map_err(|_| VerificationError::SerializationError)?;
        key.verify(&msg, signature)
    }

    fn hash(&self) -> Hash {
        sec::hash(self)
    }

    fn metadata(&self) -> MetaData {
        MetaData::empty()
    }
}

/// # SignedRecord
///
/// `SignedRecord` is a structure that represents a signed record on the blockchain. It includes the original record, its digital signature, public key, hash, and any associated metadata.
///
/// A `SignedRecord` is used to ensure that data on the blockchain is authentic and has not been tampered with. By signing the record with a private key, it can be proven that the record was created by the holder of the private key and has not been modified since it was signed. The public key is used to verify the signature and confirm the authenticity of the record.
///
/// # Type Parameters
///
/// - `R`: The type of the original record that was signed.
///
/// # Fields
///
/// - `signer`: The public key used to sign the record along with the key pair generation algorithm.
/// - `signature`: The digital signature of the record.
/// - `hash`: The hash of the record.
/// - `record`: The original record that was signed.
/// - `metadata`: Any associated metadata for the record.
///
/// # Examples
///
/// ```
/// use blockify::{sec, trans::{record::{Record, SignedRecord}}};
/// use serde::{Serialize, Deserialize};
/// use record_derive::Record;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Record, PartialEq)]
/// struct Vote {
///     session: i32,
///     choice: i32,
/// }
///
/// let keypair = sec::generate_ed25519_key_pair();
/// let original_vote = Vote { session: 0, choice: 2 };
/// let my_vote = original_vote.clone();
///
/// // Create a signed record from the original record and the keypair
/// let signed_record = my_vote.record(keypair.clone()).unwrap();
///
/// // Verify that the signed record contains the original record
/// assert_eq!(signed_record.record(), &original_vote);
///
/// // Verify that the signed record has no metadata
/// assert_eq!(signed_record.metadata(), &blockify::dat::MetaData::empty());
///
/// // Verify the authenticity of the record using the public key
/// assert!(signed_record.verify().is_ok());
/// ```

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignedRecord<R> {
    signer: PublicKey,
    signature: DigitalSignature,
    hash: Hash,
    record: R,
    metadata: MetaData,
}

impl<R: Record> SignedRecord<R> {
    pub fn new(
        record: R,
        signature: DigitalSignature,
        signer: PublicKey,
        hash: Hash,
        metadata: MetaData,
    ) -> Self {
        Self {
            record,
            signature,
            hash,
            signer,
            metadata,
        }
    }

    pub fn signature(&self) -> &DigitalSignature {
        &self.signature
    }

    pub fn record(&self) -> &R {
        &self.record
    }

    pub fn signer(&self) -> &PublicKey {
        &self.signer
    }

    pub fn keypair_algorithm(&self) -> KeyPairAlgorithm {
        self.signer.algorithm()
    }

    pub fn verify(&self) -> Result<(), VerificationError> {
        self.record.verify(self.signature(), self.signer())
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }
}
