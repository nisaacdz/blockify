use crate::record::{Record, SignedRecord};

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
    InvalidRecordInBlock(SignedRecord<R>),
    FromBlockBaseErrs(BlockBaseErrs),
    UnknownErrs,
}