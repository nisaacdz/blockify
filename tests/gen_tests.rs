#![cfg(test)]

use blockify::{record::Record, data::Metadata};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Record)]
struct Detail<T: Serialize + for<'d> Deserialize<'d>> {
    val: T,
}

#[test]
fn test_derive() {
    let keypair = blockify::generate_ed25519_keypair();
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
    let keypair = blockify::generate_ed25519_keypair();
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
