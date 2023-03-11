use std::{
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use blockify::{axs::unit::Micron, net::Peer, trans::record::Record};
use serde::{Deserialize, Serialize};

pub struct Candidate {
    id: blockify::refs::ID,
}

pub struct Voter {
    id: Vec<u8>,
    allowed_votes: blockify::axs::unit::Units,
}

impl Peer<Vote> for Voter {
    fn public_key(&self) -> &[u8] {
        &self.id
    }

    fn units(&self) -> &blockify::axs::unit::Units {
        &self.allowed_votes
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    voter_id: Vec<u8>,
    choice_candidate_id: blockify::refs::ID,
}

impl Record for Vote {
    fn metadata(&self) -> blockify::refs::MetaData {
        blockify::refs::MetaData::empty()
    }
}

pub struct Election {
    id: blockify::refs::ID,
    cost: u64,
    voters: Arc<Mutex<dyn VotersBase>>,
    candidates: Arc<Mutex<dyn CandidatesBase>>,
}

impl Election {
    pub fn id(&self) -> blockify::refs::ID {
        self.id.clone()
    }
    pub fn cost(&self) -> Micron {
        Micron::create(self.cost, self.id())
    }

    pub fn new(
        id: blockify::refs::ID,
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

trait VotersBase {
    fn can_vote(&self, voter: &Voter) -> bool;
    fn add_vote(&self, voter: &Voter, vote: &Vote) -> bool;
}

trait CandidatesBase {
    fn all_candidates(&self) -> Rc<dyn Iterator<Item = Candidate>>;
    fn add(&self, candidate: Candidate) -> bool;
    fn drop(&self, candidate: Candidate) -> bool;
}

pub struct MyVoteDB {}

impl MyVoteDB {
    pub fn new() -> Self {
        Self {}
    }
}

impl VotersBase for MyVoteDB {
    fn can_vote(&self, voter: &Voter) -> bool {
        todo!()
    }

    fn add_vote(&self, voter: &Voter, vote: &Vote) -> bool {
        todo!()
    }
}

impl CandidatesBase for MyVoteDB {
    fn all_candidates(&self) -> Rc<dyn Iterator<Item = Candidate>> {
        todo!()
    }

    fn add(&self, candidate: Candidate) -> bool {
        todo!()
    }

    fn drop(&self, candidate: Candidate) -> bool {
        todo!()
    }
}
