use crate::trans::{blocks::BlockBuilder, record::Record};

pub trait BlockVerifier {
    type Item: Record;
    fn verify(&self, block: &BlockBuilder<Self::Item>) -> VerificationResult;
}

pub struct VerificationResult;

impl VerificationResult {
    pub fn allow(&self) -> bool {
        todo!()
    }
}