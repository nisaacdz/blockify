use std::{
    collections::HashSet,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    errs::BlockifyError,
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
    pub fn publish<R: Record>(&mut self, record: SignedRecord<R>) -> Result<(), BlockifyError> {
        self.push_to_pending(&record)?;
        todo!()
    }

    pub fn push_to_pending<R: Record>(
        &mut self,
        record: &SignedRecord<R>,
    ) -> Result<(), BlockifyError> {
        let record = match serde_json::to_string(record) {
            Ok(v) => v,
            Err(_) => {
                return Err(BlockifyError::new(
                    "Can't Convert record To String: Occurring At node.rs impl for Node",
                ))
            }
        };

        match self.pending.write() {
            Err(_) => return Err(BlockifyError::new("Unable to acquire RwLock")),
            Ok(mut v) => v.append(&record),
        }
    }

    pub async fn broadcast<R: Record>(&self, block: SignedRecord<R>) {
        todo!()
    }

    pub fn poll_mem_pool<R: Record>(&self) -> Result<SignedRecord<R>, BlockifyError> {
        match self.mem_pool.write() {
            Ok(v) => match v.poll() {
                Ok(val) => match serde_json::from_str::<SignedRecord<R>>(&val) {
                    Ok(k) => return Ok(k),
                    Err(_) => return Err(BlockifyError::new("Deserialization failed")),
                },
                Err(_) => todo!(),
            },
            _ => return Err(BlockifyError::new("Cannot Acquire RwLock")),
        }
    }
}
