use crate::trans::record::{Record, SignedRecord};

pub enum Errs<'b, R: Record> {
    InvalidRecord(&'b SignedRecord<R>),
    InvalidBlockItem(&'b SignedRecord<R>),
}

pub enum GenErrs {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidSignature,
    FailedVerification,
}

pub enum BlockBaseErrs {
    NoSuchTable(String),
}

pub enum ChainBaseErrs<R: Record> {
    NoSuchChain,
    PoisonedMutex,
    VerificationFailed,
    InvalidRecordInBlock(SignedRecord<R>),
    FromBlockBaseErrs(BlockBaseErrs),
    UnknownErrs,
}

pub enum BError {
    CannotUpdateMerkleRoot,
}
