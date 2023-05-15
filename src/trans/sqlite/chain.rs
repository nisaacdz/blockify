use std::marker::PhantomData;
use diesel::prelude::*;

#[cfg(feature="chain")]


    pub struct SqliteChain<X> {
        con: SqliteConnection,
        _data: PhantomData<X>,
    }

    pub enum SqliteChainError {
        ConnectionError(ConnectionError)
    }

    impl From<ConnectionError> for SqliteChainError {
        fn from(value: ConnectionError) -> Self {
            SqliteChainError::ConnectionError(value)
        }
    }

    impl<X> SqliteChain<X> {
        pub fn new(url: &str) -> Result<Self, SqliteChainError> {
            let con = SqliteConnection::establish(url)?;
            let val = Self {
                con,
                _data: PhantomData
            };

            Ok(val)
        }

        
    }