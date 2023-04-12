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

pub fn test_co() {
    let election_id = ID::random();

    let keypair = blockify::sec::generate_key_pair();

    let units = Units::new(vec![Micron::create(1, election_id.clone())]);

    let voters = vec![sample::Voter::new(keypair.public_key().to_vec(), units)];

    let candidate = sample::Candidate::generate();
    let cid = candidate.id();

    let candidates = vec![candidate];

    let db = Arc::new(Mutex::new(sample::MyVoteDB::new(
        candidates.into_iter(),
        voters.clone().into_iter(),
    )));

    let election = sample::Election::new(election_id, 1, db.clone(), db.clone());

    let mut records = vec![];
    for voter in voters {
        records.push(voter.cast_vote(
            election.id(),
            cid.clone(),
            keypair.private_key(),
            db.clone(),
        ));
    }

    print!("{:?}", records);
}

mod sample {
    use std::{
        collections::HashSet,
        hash::Hash,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use blockify::{
        axs::unit::Micron,
        net::Peer,
        refs::{MetaData, ID},
        trans::record::{Record, SignedRecord},
    };
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct Candidate {
        id: ID,
        details: MetaData,
    }

    impl Candidate {
        pub fn generate() -> Self {
            Self {
                id: ID::random(),
                details: MetaData::empty(),
            }
        }

        pub fn id(&self) -> ID {
            self.id.clone()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Voter {
        id: Vec<u8>,
        allowed_votes: blockify::axs::unit::Units,
        details: MetaData,
    }

    impl Voter {
        pub fn new(id: Vec<u8>, allowed_votes: blockify::axs::unit::Units) -> Self {
            Self {
                id,
                allowed_votes,
                details: MetaData::empty(),
            }
        }

        pub fn cast_vote(
            &self,
            election: ID,
            choice: ID,
            key: &[u8],
            db: Arc<Mutex<dyn VotersBase>>,
        ) -> Option<SignedRecord<Vote>> {
            let vote = Vote::new(self.id.clone(), election, choice);
            let r = match vote.sign(
                self.public_key(),
                key,
                blockify::axs::algos::KeyPairAlgorithm::Ed25519,
            ) {
                Ok(v) => v,
                _ => return None,
            };

            match db.lock() {
                Ok(mut v) => match v.add_vote(r.clone()) {
                    true => Some(r),
                    false => None,
                },
                Err(_) => None,
            }
        }
    }

    impl Peer for Voter {
        fn public_key(&self) -> &[u8] {
            &self.id
        }

        fn units(&self) -> &blockify::axs::unit::Units {
            &self.allowed_votes
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Vote {
        election_id: ID,
        voter_id: Vec<u8>,
        choice_candidate_id: ID,
        details: MetaData,
    }

    impl Vote {
        pub fn new(voter_id: Vec<u8>, election_id: ID, choice_candidate_id: ID) -> Self {
            Self {
                voter_id,
                choice_candidate_id,
                election_id,
                details: MetaData::empty(),
            }
        }
    }

    impl Record for Vote {
        fn metadata(&self) -> MetaData {
            self.details.clone()
        }
    }

    #[allow(unused)]
    pub struct Election {
        id: ID,
        cost: u64,
        voters: Arc<Mutex<dyn VotersBase>>,
        candidates: Arc<Mutex<dyn CandidatesBase>>,
    }

    #[allow(unused)]
    impl Election {
        pub fn id(&self) -> ID {
            self.id.clone()
        }
        pub fn cost(&self) -> Micron {
            Micron::create(self.cost, self.id())
        }

        pub fn new(
            id: ID,
            cost: u64,
            voters: Arc<Mutex<MyVoteDB>>,
            candidates: Arc<Mutex<MyVoteDB>>,
        ) -> Self {
            Self {
                id,
                cost,
                voters,
                candidates,
            }
        }
    }

    pub trait VotersBase {
        fn can_vote(&self, voter: &Voter) -> bool;
        fn add_vote(&mut self, vote: SignedRecord<Vote>) -> bool;
    }

    pub trait CandidatesBase {
        fn all_candidates(&self) -> Rc<dyn Iterator<Item = Candidate>>;
        fn add(&mut self, candidate: Candidate) -> bool;
        fn drop(&mut self, candidate: Candidate) -> bool;
    }

    pub struct MyVoteDB {
        candiates: HashSet<Candidate>,
        voters: HashSet<Voter>,
        votes: Vec<SignedRecord<Vote>>,
    }

    impl MyVoteDB {
        pub fn new<C: Iterator<Item = Candidate>, V: Iterator<Item = Voter>>(
            candidates: C,
            voters: V,
        ) -> Self {
            Self {
                candiates: HashSet::from_iter(candidates),
                voters: HashSet::from_iter(voters),
                votes: vec![],
            }
        }
    }

    impl VotersBase for MyVoteDB {
        fn can_vote(&self, voter: &Voter) -> bool {
            self.voters.contains(voter)
        }

        fn add_vote(&mut self, vote: SignedRecord<Vote>) -> bool {
            self.votes.push(vote);
            true
        }
    }

    impl CandidatesBase for MyVoteDB {
        fn all_candidates(&self) -> Rc<dyn Iterator<Item = Candidate>> {
            Rc::new(self.candiates.clone().into_iter())
        }

        fn add(&mut self, candidate: Candidate) -> bool {
            self.candiates.insert(candidate)
        }

        fn drop(&mut self, candidate: Candidate) -> bool {
            self.candiates.remove(&candidate)
        }
    }
}
