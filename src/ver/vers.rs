use crate::trans::{block::UnchainedInstance, record::Record};

pub trait BlockVerifier {
    type Item: Record;
    fn verify(&self, block: &UnchainedInstance<Self::Item>) -> VerificationResult;
}

pub struct VerificationResult;

impl VerificationResult {
    pub fn allow(&self) -> bool {
        todo!()
    }
}
