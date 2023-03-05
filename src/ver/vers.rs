use crate::trans::{blocks::BlockBuilder, record::Record};

pub trait BlockVerifier {
    fn verify<X: Record>(&self, block: &BlockBuilder<X>) -> VerificationResult;
}

pub struct VerificationResult;

impl VerificationResult {
    pub fn allow(&self) -> bool {
        false
    }
}