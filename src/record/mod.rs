use serde::{Deserialize, Serialize};

use crate::{errs::*, gen, io::RecordBaseInsertable};

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
/// * transaction data
/// * metadata
/// * A vote
/// * smart contract state
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
///
///
/// # Example
///
/// ```
/// struct Vote {
///     voter_id: i32,
///     vote: i32,
/// }
/// ```
///
/// ```
/// impl Record for Vote {
///
/// }
/// ```
/// ```
/// fn main() {
///     let my_vote = Vote {voter_id: 1, vote: 1};
///     my_vote.
/// }
/// ```

pub trait Record: Serialize + Clone + for<'a> Deserialize<'a> {
    /// Signs the current record object using the provided `private_key`.
    ///
    /// The record object is serialized using `bincode` before signing, and the resulting
    /// signature is returned along with a clone of the original record as a `SignedRecord`.
    ///
    /// # Arguments
    ///
    /// * `private_key` - A byte slice representing the private key used to sign the record.
    ///
    /// # Returns
    ///
    /// * `Ok(SignedRecord)` - If signing is successful, returns the signed record as a
    /// `SignedRecord`.
    /// * `Err(GenErrs)` - If signing fails, returns a `GenErrs` variant representing
    /// the type of failure.
    fn sign(&self, private_key: &[u8]) -> Result<SignedRecord<Self>, GenErrs> {
        let msg = bincode::serialize(self).unwrap();
        let signature = gen::sign(&msg, private_key)?;
        Ok(SignedRecord {
            sign: signature,
            record: self.clone(),
        })
    }
    /// This function should verify that the Record object is valid
    /// and meets any necessary requirements or constraints.
    ///
    /// For example, if the Record object represents a transaction,
    /// this function might verify that the sender has sufficient funds
    /// to complete the transaction.
    fn is_valid(&self) -> bool;

    fn hash(&self) {}

    /// Returns the public key of the signer of this record.
    ///
    /// Consider adding a signer field to the implementing struct
    fn get_signer(&self) -> &[u8];

    /// Verifies the validity of `sign` for this record.
    ///
    /// # Arguments
    ///
    /// * `sign` - A reference to the signature bytes.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the signature is valid and no errors were encountered.
    /// * `Ok(false)` - If the signature is not valid but no errors were encountered.
    /// * `Err(GenErrs::InvalidSignature)` - If the value of `sign` is invalid.
    /// * `Err(GenErrs::InvalidPublicKey)` - If the public key of the signer is invalid.
    fn verify_signature(&self, sign: &[u8]) -> Result<bool, GenErrs> {
        let msg = bincode::serialize(self).unwrap();
        gen::verify_signature(&msg, sign, self.get_signer())
    }
}

const RECORDS: [&'static str; 3] = ["Record", "Signature", "Signer"];
const NAME: &'static str = "Records";

pub struct SignedRecord<R: Record> {
    sign: Vec<u8>,
    record: R,
}

impl<R: Record> SignedRecord<R> {
    pub fn get_signature(&self) -> &[u8] {
        &self.sign
    }

    pub fn get_record(&self) -> &R {
        &self.record
    }
    /// Verifies the validity of the signature for this `SignedRecord` object by
    /// calling the `verify_signature`
    /// function on the corresponding `Record` object.
    /// Returns a boolean value indicating whether the signature
    /// is valid or not.
    ///
    /// # Returns:
    /// - `true` if the signature is valid for the record and the `verify_signature` function
    /// returns `Ok(true)`.
    /// - `false` if the signature is not valid or the `verify_signature` function returns `Err(_)` or `Ok(false)`.

    pub fn verify_signature(&self) -> bool {
        match self.get_record().verify_signature(self.get_signature()) {
            Err(_) => false,
            Ok(false) => false,
            _ => true,
        }
    }
}

///
///
///
///

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
