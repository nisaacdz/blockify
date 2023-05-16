#[derive(Debug, Clone, Copy)]
pub enum DataBaseError {
    NoSuchTable,
    NoSuchKey,
    NoSuchFile,
    ConnectionFailed,
    ConnectionCannotEstablish,
}

#[derive(Debug, Clone, Copy)]
pub enum SerdeError {
    SerializationError,
    DeserializationError,
}
