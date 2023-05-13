# BLOCKIFY


A Rust blockchain library that provides the building blocks for creating a full-fledged blockchain application or platform, allowing you to focus on the higher-level features of your application without worrying about the low-level details of `block validation`, `data serialization`, and `cryptographic operations`.


# FEATURES
- **crypto**: 
- **record**:
- **blockchain**:
- **block**:
- **chain**:


# EXAMPLES

- **Record Trait** 
```
fn main() {
    use blockify::{sec, trans::record::Record};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Record)]
    struct Vote {
        session: i32,
        choice: i32,
    }

    // Generate a new keypair
    let keypair = sec::generate_ed25519_key_pair();

    // Clone the public key
    let pub_key = keypair.clone().into_public_key();

    // Create a new `Vote` instance
    let my_record = Vote {
        session: 0,
        choice: 2,
    };

    // calculate the hash of my_record
    let my_record_hash = sec::hash(&my_record);

    // sign my_record with the AuthKeyPair instance and obtain a digital signature
    let signature = my_record.sign(&keypair).unwrap();

    // verify the authencity of the digital signature
    assert!(my_record.verify(&signature, &pub_key).is_ok());

    // record the my_vote (convert it into a SignedRecord instance)
    let signed_record = my_record.record(keypair).unwrap();

    // Compare the signature of `my_record` with that inside the `SignedRecord` instance
    assert_eq!(&signature, signed_record.signature());

    // Compare the public key used to sign my_record with that inside the `SignedRecord` instance.
    assert_eq!(&pub_key, signed_record.signer());

    // Compare the hash of my_record with that inside the `SignedRecord` instance.
    assert_eq!(&my_record_hash, signed_record.hash());

    // Verfify the signature within the `SignedRecord` instance.
    assert!(signed_record.verify().is_ok());
}

```





# CONTRIBUTING

All forms of contributions are gladly welcome.


# DEPENDENCIES


# LICENSE

**MIT**