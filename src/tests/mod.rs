use serde::{Deserialize, Serialize};

use crate::{net::Entity, trans::record::Record, GenID};

#[test]
pub fn test_somthing() {
    assert_eq!(2, 2);
}

#[derive(Debug, Serialize, Deserialize)]
struct Vote {
    voterId: Vec<u8>,
    sessionId: GenID,
    choice: GenID,
    metadata: crate::MetaData,
}

impl Clone for Vote {
    fn clone(&self) -> Self {
        unsafe {
            Self { 
                voterId: self.voterId.clone(), 
                sessionId: self.sessionId.clone(), 
                choice: self.choice.clone(), 
                metadata: std::mem::copy(&self.metadata)
            }
        }
    }
}

impl Record for Vote {
    fn metadata(&self) -> crate::MetaData {
        todo!()
    }
}

struct Voter {
    public_key: Vec<u8>,
    id: GenID,
    allowed_votes: Vec<GenID>,
}

impl Entity<Vote> for Voter {
    fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}
