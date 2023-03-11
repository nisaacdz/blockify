use std::sync::{Arc, Mutex};

use blockify::{
    axs::unit::{Micron, Units},
    refs::ID,
};

mod test;

fn main() {
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
    };

    print!("{:?}", records);
}
