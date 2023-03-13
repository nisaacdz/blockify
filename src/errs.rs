use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, thiserror::Error)]
pub struct BlockifyError {
    msg: String,
}

impl std::fmt::Display for BlockifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl BlockifyError {
    pub fn emptytable() -> Self {
        Self {
            msg: String::from("Empty Table"),
        }
    }

    pub fn unknown() -> Self {
        Self {
            msg: String::from("Unknown Error"),
        }
    }

    pub fn invalid_signature() -> Self {
        Self {
            msg: String::from("Invalid Signature"),
        }
    }

    pub fn invalid_signer() -> Self {
        Self {
            msg: String::from("Invalid Public Key"),
        }
    }

    pub fn invalid_key() -> Self {
        Self {
            msg: String::from("Invalid Private Key"),
        }
    }

    pub fn failed_verification() -> Self {
        Self {
            msg: String::from("Failed Verification"),
        }
    }

    pub fn invalid_record() -> Self {
        Self {
            msg: String::from("Invalid Record"),
        }
    }

    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}
