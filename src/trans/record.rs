use serde::{Deserialize, Serialize};

use crate::{crypto::*, data::MetaData};

pub use record_derive::Record;

/// The `Record` trait provides a structure and functions for securely and transparently storing data on the blockchain.
///  
/// Any type that needs security provided by cryptographic operations can implement this trait.
///
/// `Record` contains methods for `signing`, `hashing`, `signature verification` and `recording` of records.
///
/// # Examples
///
/// ```
/// use blockify::record::Record;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Clone, Serialize, Deserialize, Record)]
/// struct Vote {
///     session: i32,
///     choice: i32,
/// }
///
/// // Generate an `ed25519` key pair
/// let keypair = blockify::generate_ed25519_key_pair();
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
    /// Attempts to convert `self` into a `SignedRecord` instance by singing it with the provided `AuthKeyPair`.
    /// 
    /// This function accepts a `MetaData` type which may be empty (i.e `MetaData::empty()`).
    ///
    /// # Returns
    ///
    /// - `Ok(SignedRecord<T>)`
    /// - `Err(SigningError)`
    ///
    fn record(
        self,
        keypair: AuthKeyPair,
        metadata: MetaData,
    ) -> Result<SignedRecord<Self>, SigningError>
    where
        Self: Serialize,
    {
        let signature = self.sign(&keypair)?;
        let hash = self.hash();
        Ok(SignedRecord::new(
            self,
            signature,
            keypair.into_public_key(),
            hash,
            metadata,
        ))
    }
    /// Signs the record with the given key and returns the signature, if the signing succeeds
    ///
    /// # Arguments
    ///
    /// * `AuthKeyPair` - The private key to use for signing.
    ///
    /// # Returns
    ///
    /// * `Ok(DigitalSignature)`
    /// * `Err(SigningError)`
    fn sign(&self, key: &AuthKeyPair) -> Result<DigitalSignature, SigningError>
    where
        Self: Serialize,
    {
        let msg = bincode::serialize(self).map_err(|_| SigningError::SerializationError)?;
        let signature = sign_msg(&msg, key)?;
        Ok(signature)
    }

    /// Attempts to verify the `DigitalSignature` for `self` with the given `PublicKey`
    ///
    /// # Arguments
    ///
    /// * `DigitalSignature`
    /// * `PublicKey`
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    /// * `Err(VerificationError)`
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
        hash(self)
    }
}


/// A `SignedRecord` is a type of data that can be added to a `block` to be put on a `blockchain`
///
/// It can be used to ensure that data in the block is authentic and has not been tampered with.
///
///
/// # Type Parameters
///
/// - `R`: The type of the original record that was signed.
///
///
/// # Examples
///
/// ```
/// use blockify::{data::MetaData, record::Record};
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
///    let keypair = blockify::generate_ed25519_key_pair();
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
///    let my_record_hash = blockify::hash(&my_record);
///
///    // sign my_record with the AuthKeyPair instance and obtain a digital signature
///    let signature = my_record.sign(&keypair).unwrap();
///
///    // verify the authencity of the digital signature
///    assert!(my_record.verify(&signature, &pub_key).is_ok());
///
///    // record the my_vote (convert it into a SignedRecord instance)
///    let signed_record = my_record.record(keypair, MetaData::empty()).unwrap();
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
