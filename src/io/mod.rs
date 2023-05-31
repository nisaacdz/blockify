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

macro_rules! impl_display_error {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
                // write!(f, "{}", res)
            }
        }

        impl std::error::Error for $type {}
    };
}

impl_display_error!(SerdeError);
