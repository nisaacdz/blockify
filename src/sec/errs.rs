pub enum Failure {
    SigningFailure(SigningFailures),
    CannotSerializeType,
}

#[derive(Debug)]
pub enum SigningFailures {
    InvalidPrivateKey,
    InvalidPublicKey,
}
