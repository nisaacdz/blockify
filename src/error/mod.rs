
use super::impl_display_error;

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

impl_display_error!(SerdeError);
impl_display_error!(DataBaseError);


mod tests {

    #[test]
    fn test_diplay() {
        println!("{}", crate::error::SerdeError::SerializationError)
    }
}