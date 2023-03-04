
pub enum GenErrs {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidSignature,
    FailedVerification,
}

pub enum BlockBaseErrs<'a> {
    NoSuchTable(&'a str),
}
