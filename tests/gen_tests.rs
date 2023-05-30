#![cfg(test)]

use blockify::{record::Record, data::Metadata};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Detail<T> {
    val: T,
}

impl<T: Serialize> Record for Detail<T> {
    fn sign(
        &self,
        key: &blockify::AuthKeyPair,
    ) -> Result<blockify::DigitalSignature, blockify::SigningError> {
        let msg = blockify::serialize(self)?;
        let signature = blockify::sign_msg(&msg, key)?;
        Ok(signature)
    }

    fn verify(
        &self,
        signature: &blockify::DigitalSignature,
        key: &blockify::PublicKey,
    ) -> Result<(), blockify::VerificationError> {
        let msg = blockify::serialize(self).map_err(|_| blockify::VerificationError::SerdeError)?;
        key.verify(&msg, signature)
    }

    fn record(
        self,
        keypair: blockify::AuthKeyPair,
        metadata: blockify::data::Metadata,
    ) -> Result<blockify::record::SignedRecord<Self>, blockify::SigningError> {
        let signature = self.sign(&keypair)?;
        let hash = self.hash();
        Ok(blockify::record::SignedRecord::new(
            self,
            signature,
            keypair.into_public_key(),
            hash,
            metadata,
        ))
    }

    fn hash(&self) -> blockify::Hash {
        blockify::hash(self)
    }
}

#[test]
fn test_derive() {
    let keypair = blockify::generate_ed25519_key_pair();
    let value = Detail {
        val: String::from("Hello, World!"),
    };
    let hash = value.hash();
    let signature = value.sign(&keypair).unwrap();
    let record = value.record(keypair, Metadata::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert_eq!(&Metadata::empty(), record.metadata());
    assert!(record.verify().is_ok());
}

#[test]
fn test_record() {
    let keypair = blockify::generate_ed25519_key_pair();
    let value = Detail {
        val: String::from("Hello, World!"),
    };
    let hash = value.hash();
    let signature = value.sign(&keypair).unwrap();
    let record = value.record(keypair, Metadata::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert_eq!(&Metadata::empty(), record.metadata());
    assert!(record.verify().is_ok());
}
