pub enum DataBaseError {
    NoSuchTable,
    NoSuchKey,
    ConnectionFailure,
    ConnectionCannotEstablish,
}

pub enum SerdeError {
    SerializationError,
    DeserializationError,
}

