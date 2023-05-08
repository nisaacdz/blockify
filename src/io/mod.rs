pub enum DataBaseError {
    NoSuchTable,
    NoSuchKey,
    NoSuchFile,
    ConnectionFailed,
    ConnectionCannotEstablish,
}

pub enum SerdeError {
    SerializationError,
    DeserializationError,
}
