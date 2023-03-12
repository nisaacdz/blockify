use serde::{Deserialize, Serialize};

use crate::{
    io::RecordBaseInsertable,
    refs::MetaData,
    sec::{self, errs::Failure},
};

use super::algos::KeyPairAlgorithm;

/// # Disclaimer
/// In this context, a `Record` object is any data or information that needs to be
///  securely and `transparently` stored on the blockchain.
///
/// `transparently` in this sense does not mean that the data necessarily needs to be
/// viewable by uninvolved parties but it should be `provable` and `verifyable`
///
/// `provable` means it should be possible to demonstrate or confirm the authenticity
/// of the data.
/// `verifyable` means it should be possible to demonstrate or confirm the occurance of the
/// record.
///
/// # Examples of Records
/// * A transaction data
/// * A metadata
/// * A vote
/// * A smart contract state
/// * Any other type of information that needs to be recorded and verified on a blockchain.
///
/// # How to Use
/// To use the blockchain library, users will need to implement the Record trait on their type,
/// which will define the structure and properties of the record they want to store on the
/// blockchain.
///
/// # What this trait contains
/// This trait includes methods for serializing and deserializing the
/// data, as well as for verifying the integrity and authenticity of the record on the
/// blockchain.
///
/// By implementing the Record trait on their type, users can ensure that their
/// data is securely and transparently recorded on the blockchain, with all the benefits
/// of decentralization, transparency, and immutability that blockchain technology provides.

pub trait Record: Serialize + Clone + for<'de> Deserialize<'de> {
    fn sign(
        &self,
        public_key: &[u8],
        private_key: &[u8],
        algorithm: KeyPairAlgorithm,
    ) -> Result<SignedRecord<Self>, Failure> {
        sec::sign(self, public_key, private_key, algorithm, self.metadata())
    }

    fn hash(&self) -> Vec<u8> {
        sec::hash(self)
    }

    fn metadata(&self) -> MetaData;
}

const RECORDS: [&'static str; 3] = ["Record", "Signature", "Signer"];
const NAME: &'static str = "Records";

#[derive(Serialize, Debug, Clone)]
pub struct SignedRecord<R: Record> {
    signer: Vec<u8>,
    signature: Vec<u8>,
    hash: Vec<u8>,
    record: R,
    algorithm: KeyPairAlgorithm,
    metadata: MetaData,
}

impl<R: Record> SignedRecord<R> {
    pub fn new(
        record: R,
        signature: Vec<u8>,
        algorithm: KeyPairAlgorithm,
        public_key: &[u8],
        hash: Vec<u8>,
        metadata: MetaData,
    ) -> Self {
        Self {
            record,
            signature,
            hash,
            signer: public_key.to_vec(),
            algorithm,
            metadata,
        }
    }

    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    pub fn record(&self) -> &R {
        &self.record
    }

    pub fn signer(&self) -> &[u8] {
        &self.signer
    }

    pub fn algorithm(&self) -> KeyPairAlgorithm {
        self.algorithm
    }

    /// Verifies the validity of the signature for this `SignedRecord` object.
    /// Returns a boolean value indicating whether the signature
    /// is valid or not.
    ///
    /// # Returns:
    /// - `true` if the signature is valid for the record and the `verify_signature` function
    /// returns `Ok(true)`.
    /// - `false` if the signature is not valid or the `verify_signature` returns `Err(_)` or `Ok(false)`.
    ///

    pub fn verify_signature(&self) -> Result<(), ring::error::Unspecified> {
        let msg = bincode::serialize(self.record()).unwrap();
        sec::verify_signature(&msg, &self.signature, &self.signer, self.algorithm)
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn metadata(&self) -> MetaData {
        self.record().metadata()
    }
}

impl<R: Record> RecordBaseInsertable<R> for SignedRecord<R> {
    fn name() -> &'static str {
        &NAME
    }

    fn columns() -> &'static [&'static str] {
        &RECORDS
    }

    fn record(&self) -> &R {
        &self.record
    }
}
