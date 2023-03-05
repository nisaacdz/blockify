use std::sync::{Arc, Mutex};

use crate::{trans::record::{Record, SignedRecord}, sec::errs::Failure, ver::vers::BlockVerifier};


/// Entity represent parties within a blockchain network that can execute
/// the following abilites:
///
pub trait Entity<R: Record> {
    fn public_key(&self) -> &[u8];
    fn sign_record(&self, record: R, public_key: &[u8], private_key: &[u8], algo: &'static dyn ring::signature::VerificationAlgorithm) -> Result<SignedRecord<R>, Failure> {
        record.sign(public_key, private_key, algo)
    }
}

pub trait Pusher<V: BlockVerifier> {
    fn get_verifier(&self) -> Arc<Mutex<V>>;
}
