use blockify::{
    block::UnchainedInstance,
    chain::Chain,
    data::MetaData,
    record::{Record, SignedRecord},
    SqliteChain,
};
use serde::{Deserialize, Serialize};

#[derive(Record, Clone, Serialize, Deserialize)]
struct Vote {
    data: String,
}

impl Vote {
    pub fn new(data: &str) -> Self {
        Vote { data: data.into() }
    }
}

fn main() {
    let chain_url = "src/trans/sqlite/";
    let datas1 = vec!["abcd", "efgh", "ijkl"];
    let datas2 = vec!["mnop", "qrst", "uvwx"];
    let keypair = crate::generate_ed25519_key_pair();
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

    let mut block1 = UnchainedInstance::new(MetaData::empty());
    let mut block2 = UnchainedInstance::new(MetaData::empty());

    for record in records1 {
        block1.push(record);
    }

    for record in records2 {
        block2.push(record);
    }

    let mut chain =
        SqliteChain::new(chain_url).expect("sqlite connection cannot be established");
    chain.append(&block1).expect("block1 append erred");
    chain.append(&block2).expect("block2 append erred");
}