use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyPairAlgorithm {
    Ed25519,
    Ecdsa,
}
