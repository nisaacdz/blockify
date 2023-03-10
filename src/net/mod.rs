use crate::{
    sec::errs::Failure,
    trans::record::{Record, SignedRecord}, refs::Unit,
};

pub mod node;

pub trait Peer<R: Record> {
    fn public_key(&self) -> &[u8];
    fn units(&self) -> &Unit;
    fn sign_record(
        &self,
        record: R,
        public_key: &[u8],
        private_key: &[u8],
        algo: &'static dyn ring::signature::VerificationAlgorithm,
    ) -> Result<SignedRecord<R>, Failure> {
        record.sign(public_key, private_key, algo)
    }
}

pub trait Pusher {
    fn push(&self) -> bool;
}
