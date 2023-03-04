use crate::{record::{Record, SignedRecord}, errs::GenErrs};

pub mod node;



/// Entity represent parties within a blockchain network that can execute
/// the following abilites:
///
pub trait Entity<R: Record> {
    fn public_key(&self) -> &[u8];
    fn sign_record(&self, record: R, key: &[u8]) -> Result<SignedRecord<R>, GenErrs> {
        record.sign(key)
    }

    /// [Click Here](#verify_signature) for full details about return value

    fn verify_signature(&self, r: R, sign: &[u8]) -> Result<bool, GenErrs> {
        r.verify_signature(sign)
    }
}
