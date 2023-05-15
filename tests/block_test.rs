#![cfg(test)]
#![cfg(feature = "full")]

#[test]
fn test_blocks() {
    use blockify::{data::MetaData, record::Record};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Record)]
    struct Vote {
        data: String,
    }
    let keypair = blockify::generate_ed25519_key_pair();
    let value = Vote {
        data: String::from("Hello, World!"),
    };
    let hash = value.hash();
    let signature = value.sign(&keypair).unwrap();
    let record = value.record(keypair, MetaData::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert_eq!(&MetaData::empty(), record.metadata());
    assert!(record.verify().is_ok());

}