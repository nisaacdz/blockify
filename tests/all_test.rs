#![cfg(test)]

use blockify::{
    block::{Block, LocalInstance, UnchainedInstance},
    chain::Chain,
    data::Metadata,
    record::{Record, SignedRecord},
    SqliteChain,
};
use serde::{Deserialize, Serialize};

// Deriving Record does the magic
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

#[test]
fn start() {
    let contract = MarriageContract::new("John", "Julie");
    let keypair = blockify::generate_ed25519_key_pair();
    let signature = contract.sign(&keypair).unwrap();
    let hash = contract.hash();
    let record = contract.record(keypair, Metadata::empty()).unwrap();

    assert_eq!(&hash, record.hash());
    assert_eq!(&signature, record.signature());
    assert!(record.verify().is_ok());

    let mut pool = LocalInstance::new(Metadata::empty(), 0);
    let all_records = MarriageContract::generate_records(10);
    all_records
        .clone()
        .into_iter()
        .for_each(|record| pool.append(record).unwrap());

    let chain_url = "target2/tests/marriagecontractchain2/";
    std::fs::create_dir_all(chain_url).expect("could create directories");

    let mut chain = SqliteChain::new(chain_url).unwrap();
    let position = chain.append(&pool).expect("Error appending to SqliteChain");
    let block = position
        .block(&chain)
        .expect("Error getting block by position");

    assert_eq!(
        &all_records,
        &*block
            .records()
            .expect("Error retrieving records from block")
    );
}
