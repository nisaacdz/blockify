use crate::trans::{blocks::Block, record::Record};

pub trait BlockVerifier {
    type Item: Record;
    fn verify(&self, block: &Block<Self::Item>) -> VerificationResult;
}

pub struct VerificationResult;

impl VerificationResult {
    pub fn allow(&self) -> bool {
        todo!()
    }
}
