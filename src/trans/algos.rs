use serde::{Serialize, Deserialize};

use super::record::{SignedRecord, Record};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyPairAlgorithm {
    ED25519,
}
