#![cfg(test)]

#![cfg(feature = "full")]

#[test]
fn test_derive() {
    use blockify::trans::record::Record;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Record)]
    struct Detail<T> {
        val: T,
    }
}

#[test]
fn test_record() {
    use blockify::{trans::record::Record, crypto, data::MetaData};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Record)]
    struct Detail<T> {
        val: T,
    }
    let keypair = crypto::generate_ed25519_key_pair();
    let value = Detail { val: String::from("Hello, World!") };
    let hash = value.hash();
    let signature = value.sign(&keypair).unwrap();
    let record = value.record(keypair, MetaData::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert_eq!(&MetaData::empty(), record.metadata());
    assert!(record.verify().is_ok());
}
