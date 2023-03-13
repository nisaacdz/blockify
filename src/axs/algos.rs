use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyPairAlgorithm {
    Ed25519,
    Ecdsa,
}
