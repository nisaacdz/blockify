use std::sync::{Arc, Mutex};

use blockify::{
    axs::{
        algos::KeyPairAlgorithm,
        unit::{Micron, Units},
    },
    refs::ID,
    trans::record::{Record, SignedRecord},
};
use serde::{Deserialize, Serialize};

mod test;

fn main() {
    my_func()
}

fn my_func() {
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    struct Trans(i32, String, bool);

    impl Record for Trans {
        fn metadata(&self) -> blockify::refs::MetaData {
            blockify::refs::MetaData::empty()
        }
    }

    let kp = blockify::sec::generate_key_pair();
    let trans = Trans(1, "Hello".to_owned(), false);

    let sg = trans
        .sign(kp.public_key(), kp.private_key(), KeyPairAlgorithm::Ed25519)
        .unwrap();
    let ss = serde_json::to_string(&sg).unwrap();
    let res = match serde_json::from_str::<SignedRecord<Trans>>(&ss) {
        Ok(v) => v,
        Err(_) => panic!("Failure !"),
    };

    res.verify_signature().unwrap();

    assert_eq!(sg, res);
}

fn test_co() {
    let election_id = ID::random();

    let keypair = blockify::sec::generate_key_pair();

    let units = Units::new(vec![Micron::create(1, election_id.clone())]);

    let voters = vec![test::Voter::new(keypair.public_key().to_vec(), units)];

    let candidate = test::Candidate::generate();
    let cid = candidate.id();

    let candidates = vec![candidate];

    let db = Arc::new(Mutex::new(test::MyVoteDB::new(
        candidates.into_iter(),
        voters.clone().into_iter(),
    )));

    let election = test::Election::new(election_id, 1, db.clone(), db.clone());

    let mut records = vec![];
    for voter in voters {
        records.push(voter.cast_vote(cid.clone(), keypair.private_key(), db.clone()));
    }

    print!("{:?}", records);
}
