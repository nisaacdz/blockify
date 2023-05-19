use blockify::{
    block::{Block, UnchainedInstance},
    chain::Chain,
    data::MetaData,
    record::Record,
    SqliteChain,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Record, Serialize, Deserialize, PartialEq, Eq)]
struct Data {
    data: String,
}

impl Data {
    pub fn new(data: &str) -> Self {
        Data { data: data.into() }
    }
}

fn main() {
    // folder path for the chain and blocks
    let chain_url = "target2/tmp/bin/main/";
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
        .map(|w| Data::new(w).record(keypair.clone(), MetaData::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();
    let records2 = datas2
        .into_iter()
        .map(|w| Data::new(w).record(keypair.clone(), MetaData::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();

    // create two block builders `UnchainedInstance`'s with nonce and empty metadata
    let mut builder1 = UnchainedInstance::new(MetaData::empty(), 0);
    let mut builder2 = UnchainedInstance::new(MetaData::empty(), 1);

    // push the two vec's content into each UnchainedInstance
    for record in records1 {
        builder1.push(record);
    }

    for record in records2 {
        builder2.push(record);
    }

    // To build an SqliteChain (BlockChain that stores data in sqlite database),
    // pass the url to `new()`
    let mut chain = SqliteChain::new(chain_url).expect("sqlite connection cannot be established");

    // append the two UnchainedInstance's into the blockchain and obtain an a ChainedInstance
    let instance1 = chain.append(&builder1).expect("builder1 append erred");
    let instance2 = chain.append(&builder2).expect("builder2 append erred");

    // Deserialize first block from the blockchain
    let mut block1 = chain
        .block_at(instance1.position())
        .expect("couldn't retrieve block1");

    // Deserialize second block from the blockchain
    let mut block2 = chain
        .block_at(instance2.position())
        .expect("couldn't retrieve block2");

    // compare the hash, merkle_root, prev_hash, etc of block1 and instance1
    assert!(block1.validate(&instance1).is_ok());

    // compare the hash, merkle_root, prev_hash, etc of block2 and instance2
    assert!(block2.validate(&instance2).is_ok());

    // retrieve the original records from the blocks
    let records_from_block1 = block1
        .records()
        .expect("couldn't retrieve records from block1");
    assert_eq!(builder1.records().as_slice(), &*records_from_block1);

    let records_from_block2 = block2
        .records()
        .expect("couldn't retrieve records from block2");

    // Assert the equality of the records from the two blocks
    assert_eq!(builder1.records().as_slice(), &*records_from_block1);
    assert_eq!(builder2.records().as_slice(), &*records_from_block2);
}
