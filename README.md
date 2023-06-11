# BLOCKIFY

[![Rayon crate](https://img.shields.io/crates/v/blockify.svg)](https://crates.io/crates/blockify)
[![Rayon documentation](https://docs.rs/blockify/badge.svg)](https://docs.rs/blockify)

A Rust blockchain library that provides the building blocks for creating a full-fledged blockchain application or platform, allowing you to focus on the higher-level features of your application without worrying about the low-level details of `block validation`, `data serialization`, `blockchain technology`, and `cryptographic operations`.


# USEFUL FEATURES
- Generating cryptographic key pairs
- Signing data with cryptographic keys
- Serializing data into bytes
- Hashing different kinds of data
- Building and blockchains
- Building and deploying blocks into blockchains
- Architecture for building fully-fledged blockchain applications


# USAGE

- **Example**
```
    use blockify::{
    block::{LocalInstance, UnchainedInstance, Block},
    data::Metadata,
    record::{Record, SignedRecord}, SqliteChain, chain::Chain,
    };
    use serde::{Deserialize, Serialize};

    // Deriving `Record` does the magic 
    #[derive(Clone, Serialize, Deserialize, Record, Debug, PartialEq)]
    pub struct MarriageContract {
        bride_name: String,
        groom_name: String,
    }

    impl MarriageContract {
        pub fn new(bride_name: &str, groom_name: &str) -> Self {
            let (bride_name, groom_name) = (bride_name.to_owned(), groom_name.to_owned());
            Self {
                bride_name,
                groom_name,
            }
        }

        pub fn generate() -> Self {
            Self {
                bride_name: "Julian".to_owned(),
                groom_name: "Jolie".to_owned(),
            }
        }

        pub fn generate_records(amount: usize) -> Vec<SignedRecord<Self>> {
            let mut res = Vec::with_capacity(amount);
            (0..amount).for_each(|_| {
                match Self::generate().record(blockify::generate_ed25519_key_pair(), Default::default())
                {
                    Ok(v) => res.push(v),
                    Err(_) => unreachable!("Error occurs"),
                }
            });
            res
        }

    }
```

- **Creating `Records` and `SignedRecords`**
```
    let contract = MarriageContract::new("John", "Julie");
    let keypair = blockify::generate_ed25519_keypair();
    let signature = contract.sign(&keypair).unwrap();
    let hash = contract.hash();
    let record = contract.record(keypair, Metadata::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert!(record.verify().is_ok());
```


- **Assembling a `Block`**
```
    let mut pool = LocalInstance::new(Metadata::empty(), 0);
    let all_records = MarriageContract::generate_records(10);
    all_records.clone().into_iter().for_each(|record| pool.append(record).unwrap());
```

- **`SqliteBlock` and `SqliteChain`**
```
    let chain_url = "target2/tests/marriagecontractchain/";
    std::fs::create_dir_all(chain_url).expect("could initialize directories");
    
    let mut chain = SqliteChain::new(chain_url).unwrap();
    let position = chain.append(&pool).expect("Error appending to SqliteChain");
    let block = position.block(&chain).expect("Error getting block by position");

    assert_eq!(&all_records, &*block.records().expect("Error retrieving records from block"));
```

# CONTRIBUTING

All forms of contributions are gladly welcome.

# DEPENDENCIES

# LICENSE

**MIT**
