#![cfg(test)]

use blockify::{block::{LocalInstance, Block, UnchainedInstance}, record::Record, data::Metadata, SqliteChain, chain::Chain};
use serde::{Serialize, Deserialize};

#[test]
fn test_blocks() {
    #[derive(Debug, Clone, Record, Serialize, Deserialize, PartialEq, Eq)]
    struct Data {
        data: String,
    }

    impl Data {
        pub fn new(data: &str) -> Self {
            Data { data: data.into() }
        }
    }

    fn start() {
        // folder path for the chain and blocks
        let chain_url = "target2/tests/sample2/";
        // create the folder if it is absent
        std::fs::create_dir_all(chain_url).expect("could not create chain_url");
        // prepare strings for use in creating `Data` instances
        let datas1 = vec!["abcd", "efgh", "ijkl"];
        let datas2 = vec!["mnop", "qrst", "uvwx"];

        let keypair = blockify::generate_ed25519_key_pair();
        // create Data instances from the strings, sign them into SignedRecord's
        // and collect them into two vectors.
        let records1 = datas1
            .into_iter()
            .map(|w| Data::new(w).record(keypair.clone(), Metadata::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<_>>();
        let records2 = datas2
            .into_iter()
            .map(|w| Data::new(w).record(keypair.clone(), Metadata::empty()))
            .filter(|r| r.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<_>>();

        // create two block builders `UnchainedInstance`'s with nonce and empty metadata
        let mut builder1 = LocalInstance::new(Metadata::empty(), 0);
        let mut builder2 = LocalInstance::new(Metadata::empty(), 1);

        // push the two vec's content into each UnchainedInstance
        for record in records1 {
            builder1.push(record);
        }

        for record in records2 {
            builder2.push(record);
        }

        // To build an SqliteChain (BlockChain that stores data in sqlite database),
        // pass the url to `new()`
        let mut chain =
            SqliteChain::new(chain_url).expect("sqlite connection cannot be established");

        // append the two UnchainedInstance's into the blockchain and obtain an a ChainedInstance
        let instance1 = chain.append(&builder1).expect("builder1 append erred");
        let instance2 = chain.append(&builder2).expect("builder2 append erred");

        // Get the blocks in the blockchain by means of the PositionInstance values
        let block1 = instance1.block(&chain).expect("couldn't retrieve block1");
        let block2 = instance2.block(&chain).expect("couldn't retrieve block2");

        // retrieve the original records from the blocks
        let records_from_block1 = block1
            .records()
            .expect("couldn't retrieve records from block1");
        assert_eq!(builder1.records().unwrap().as_slice(), &*records_from_block1);

        let records_from_block2 = block2
            .records()
            .expect("couldn't retrieve records from block2");

        // Assert the equality of the records from the two blocks
        assert_eq!(builder1.records().unwrap().as_slice(), &*records_from_block1);
        assert_eq!(builder2.records().unwrap().as_slice(), &*records_from_block2);
    }
    start()
}


#[test]
fn test_2() {
    use blockify::{
        block::{LocalInstance, UnchainedInstance},
        data::Metadata,
        record::{Record, SignedRecord}, SqliteChain, chain::Chain,
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
    
    fn main() {
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
        all_records.clone().into_iter().for_each(|record| pool.append(record).unwrap());
    
        let mut chain = SqliteChain::new("target2/tests/marriagecontractchain/").unwrap();
        let position = chain.append(&pool).expect("Error appending to SqliteChain");
        let block = position.block(&chain).expect("Error getting block by position");
    
        assert_eq!(&all_records, &*block.records().expect("Error retrieving records from block"));
    }
    
    main()
}
