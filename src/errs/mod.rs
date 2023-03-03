pub enum SignErrs {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidSignature,
    FailedVerification,
}

pub enum BlockBaseErrs<'a> {
    NoSuchTable(&'a str),
}
