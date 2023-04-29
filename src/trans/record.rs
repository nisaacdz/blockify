use serde::{Deserialize, Serialize};

use crate::{
    axs::detail::MetaData,
    sec::{self, crypto::*, SigningError, VerificationError},
};

#[cfg(feature = "derive")]
pub use record_derive::Record;

/// # Record
///
/// The `Record` trait provides a structure and properties
/// for securely and transparently storing data on the blockchain.
///  
/// Any type that needs security provided by cryptographic operations can implement this trait.
///
///
/// # Methods
///
/// * `record` - converts `self` into a `SignedRecord`.
/// * `sign` - signs the record and returns a `DigitalSignature`.
/// * `verify` - compares a `DigitalSignature` with one from `sign`.
/// * `hash` - computes the hash of the record `self`
/// * `metadata` - computes and returns any associated metadata.
///
///
/// # Examples
///
/// ```
/// use blockify::{sec, trans::record::Record};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Clone, Serialize, Deserialize, Record)]
/// struct Vote {
///     session: i32,
///     choice: i32,
/// }
///
/// // Generate an `ed25519` key pair
/// let keypair = sec::generate_ed25519_key_pair();
///
/// // Create a `Vote` instance
/// let my_record = Vote { session: 0, choice: 2 };
///
/// // Sign `my_record` and obtain a `DigitalSignature`
/// let signature = my_record.sign(&keypair).unwrap();
///
/// // Verify the signature with the trait method `verify`
/// assert!(my_record.verify(&signature, &keypair.into_public_key()).is_ok())
/// ```

pub trait Record: Sized {
    /// converts `self` into a `SignedRecord` instance by singing it with the provided key pair
    ///
    /// # Arguments
    ///
    /// `AuthKeyPair` - The Keypair for the signing.
    ///
    /// # Returns
    ///
    /// - `Ok()` - A `SignedRecord<T>` instance.
    /// - `Err()` - A `SigningError` instance.
    ///
    fn record(self, keypair: AuthKeyPair) -> Result<SignedRecord<Self>, SigningError>
    where
        Self: Serialize,
    {
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
    /// Signs the record with the given key and returns a digital signature.
    ///
    /// # Arguments
    ///
    /// * `key` - The private key to use for signing.
    ///
    /// # Returns
    ///
    /// * `Ok(signature)` - A `DigitalSignature` instance representing the signature of the record.
    /// * `Err(error)` - A `SigningError` instance describing the error that occurred during signing.

    fn sign(&self, key: &AuthKeyPair) -> Result<DigitalSignature, SigningError>
    where
        Self: Serialize,
    {
        let msg = bincode::serialize(self).map_err(|_| SigningError::SerializationError)?;
        let signature = sec::sign_msg(&msg, key)?;
        Ok(signature)
    }

    /// Returns `Ok(())` if signature verification succeeds or a `VerificationError` if it fails.
    ///
    /// # Arguments
    ///
    /// * `signature` - The digital signature to verify.
    /// * `key` - The public key to use for verification.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the signature verification succeeds.
    /// * `Err(error)` - A `VerificationError` instance describing the error that occurred during verification.

    fn verify(&self, signature: &DigitalSignature, key: &PublicKey) -> Result<(), VerificationError>
    where
        Self: Serialize,
    {
        let msg = bincode::serialize(self).map_err(|_| VerificationError::SerializationError)?;
        key.verify(&msg, signature)
    }

    /// Computes and returns the hash of the record
    fn hash(&self) -> Hash
    where
        Self: Serialize,
    {
        sec::hash(self)
    }

    // Computes and returns any associated metadata
    fn metadata(&self) -> MetaData {
        MetaData::empty()
    }
}

/// A `SignedRecord` is a type of `Record` that can be added to a `block` to be put on a `blockchain`
///
/// It can be used to ensure that data in the block is authentic and has not been tampered with.
///
///
/// # Type Parameters
///
/// - `R`: The type of the original record that was signed.
///
///
/// # Fields
///
/// - `signer` - The public key of the signer of the record
/// - `signature` - The digital signature of the record
/// - `hash` - The hash of the record.
/// - `record` - The original record that was signed.
/// - `metadata` - Any associated metadata for the record (time, place, etc).
///
///
/// # Examples
///
/// ```
/// use blockify::{sec, trans::record::Record};
/// use serde::{Deserialize, Serialize};
///
/// fn main() {
///    #[derive(Clone, Serialize, Deserialize, Record)]
///    struct Vote {
///        session: i32,
///        choice: i32,
///    }
///
///    // Generate a new keypair
///    let keypair = sec::generate_ed25519_key_pair();
///
///    // Clone the public key
///    let pub_key = keypair.clone().into_public_key();
///
///    // Create a new `Vote` instance
///    let my_record = Vote {
///        session: 0,
///        choice: 2,
///    };
///
///    // calculate the hash of my_record
///    let my_record_hash = sec::hash(&my_record);
///
///    // sign my_record with the AuthKeyPair instance and obtain a digital signature
///    let signature = my_record.sign(&keypair).unwrap();
///
///    // verify the authencity of the digital signature
///    assert!(my_record.verify(&signature, &pub_key).is_ok());
///
///    // record the my_vote (convert it into a SignedRecord instance)
///    let signed_record = my_record.record(keypair).unwrap();
///
///    // Compare the signature of `my_record` with that inside the `SignedRecord` instance
///    assert_eq!(&signature, signed_record.signature());
///
///    // Compare the public key used to sign my_record with that inside the `SignedRecord` instance.
///    assert_eq!(&pub_key, signed_record.signer());
///
///    // Compare the hash of my_record with that inside the `SignedRecord` instance.
///    assert_eq!(&my_record_hash, signed_record.hash());
///
///    // Verify the validity of the signature within the `SignedRecord` instance.
///    assert!(signed_record.verify().is_ok());
///}
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
    /// Creates and returns a new `SignedRecord` instance with the given values.
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

    /// Returns a reference to the `DigitalSignature` on this `SignedRecord` instance
    pub fn signature(&self) -> &DigitalSignature {
        &self.signature
    }
    /// Returns a reference to the `Record` inside this `SignedRecord` instance
    pub fn record(&self) -> &R {
        &self.record
    }
    /// Returns a reference to the public key used to sign this `SignedRecord` instance
    pub fn signer(&self) -> &PublicKey {
        &self.signer
    }
    /// Returns a reference to the keypair algorithm used to sign this `SignedRecord` instance
    pub fn keypair_algorithm(&self) -> KeyPairAlgorithm {
        self.signer.algorithm()
    }

    // Returns a reference to the hash of `Record` stored within this `SignedRecord` instance
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }
}

impl<R: Record + Serialize> SignedRecord<R> {
    /// Verifies the validity of the `DigitalSignature` within this `SignedRecord` instance for the `Record` it holds.
    pub fn verify(&self) -> Result<(), VerificationError> {
        self.record.verify(self.signature(), self.signer())
    }
}
