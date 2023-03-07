use std::fs::Metadata;

use serde::{Deserialize, Serialize};

use crate::{net::Entity, trans::record::Record, GenID};

#[test]
pub fn test_somthing() {
    assert_eq!(2, 2);
}

struct Vote {
    voterId: Vec<u8>,
    sessionId: GenID,
    choice: GenID,
    metadata: crate::MetaData,
}

struct Voter {
    public_key: Vec<u8>,
    id: GenID,
    allowed_votes: Vec<GenID>,
}
