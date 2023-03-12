use std::{
    collections::HashSet,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    io::{MemPool, NodeRecord},
    refs::ID,
    trans::{
        chain::Chain,
        record::{Record, SignedRecord},
    },
};

use super::{Miner, Peer};

#[derive(Clone, Debug)]
pub struct NodeId {
    pub id: ID,
    pub address: String,
}

pub struct Node {
    pub chain: Arc<Mutex<Chain>>,

    pub id: NodeId,

    pub peers: HashSet<Arc<RwLock<dyn Peer>>>,

    pub mem_pool: Arc<RwLock<dyn MemPool>>,

    pub pending: Arc<RwLock<dyn MemPool>>,

    pub transactions: Arc<Mutex<dyn NodeRecord>>,

    pub local_chain: Chain,

    pub network: Arc<Mutex<Vec<NodeId>>>,

    pub miners: Arc<RwLock<Vec<Box<dyn Miner>>>>,

    pub units: crate::axs::unit::UnitManager,
}

impl Node {
    pub async fn broadcast<R: Record>(&self, block: SignedRecord<R>) {
        todo!()
    }

    pub fn poll_mem_pool<R: Record>(&self) -> Option<SignedRecord<R>> {
        match self.mem_pool.write() {
            Ok(v) => match serde_json::from_str::<SignedRecord<R>>(&v.poll()) {
                Ok(k) => return Some(k),
                Err(_) => return None,
            },
            _ => return None,
        }
    }
}
