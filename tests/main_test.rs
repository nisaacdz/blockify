use blockify::{
    block::{Block, UnchainedInstance},
    chain::Chain,
    data::MetaData,
    record::{Record, SignedRecord},
    SqliteChain,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Record, Clone, Serialize, Deserialize, PartialEq)]
struct Vote {
    data: String,
}

impl Vote {
    fn new(data: &str) -> Self {
        Vote { data: data.into() }
    }
}

#[test]
fn test_main() {
    let chain_url = "target2/main/sqlite/";
    std::fs::create_dir_all(chain_url).expect("could not create chain_url");
    let datas1 = vec!["abcd", "efgh", "ijkl"];
    let datas2 = vec!["mnop", "qrst", "uvwx"];
    let keypair = blockify::generate_ed25519_key_pair();
    let records1 = datas1
        .into_iter()
        .map(|w| Vote::new(w).record(keypair.clone(), MetaData::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<SignedRecord<Vote>>>();
    let records2 = datas2
        .into_iter()
        .map(|w| Vote::new(w).record(keypair.clone(), MetaData::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<SignedRecord<Vote>>>();

    let mut builder1 = UnchainedInstance::new(MetaData::empty(), 0);
    let mut builder2 = UnchainedInstance::new(MetaData::empty(), 1);

    for record in records1 {
        builder1.push(record);
    }

    for record in records2 {
        builder2.push(record);
    }

    let mut chain = SqliteChain::new(chain_url).expect("sqlite connection cannot be established");
    let instance1 = chain.append(&builder1).expect("builder1 append erred");
    let instance2 = chain.append(&builder2).expect("builder2 append erred");

    let block1 = chain
        .block_at(instance1.position())
        .expect("couldn't retrieve block1");
    let block2 = chain
        .block_at(instance2.position())
        .expect("couldn't retrieve block2");

    assert!(block1.validate(&instance1).is_ok());
    assert!(block2.validate(&instance2).is_ok());

    let records_from_block1 = block1
        .records()
        .expect("couldn't retrieve records from block1");
    assert_eq!(builder1.records().as_slice(), &*records_from_block1);

    let records_from_block2 = block2
        .records()
        .expect("couldn't retrieve records from block2");
    assert_eq!(builder2.records().as_slice(), &*records_from_block2);
}
