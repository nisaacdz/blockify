#[derive(Debug, Clone)]
pub enum Failure {
    SigningFailure(SigningFailures),
    CannotSerializeType,
}

#[derive(Debug, Clone)]
pub enum SigningFailures {
    InvalidPrivateKey,
    InvalidPublicKey,
}
