use std::{sync::{Mutex, RwLock, Arc}, collections::HashSet};

use crate::{trans::{chain::Chain, record::{SignedRecord, Record}}, io::{NodeRecord, MemPool}, refs::ID};

use super::{Miner, Peer};

#[derive(Clone, Debug)]
pub struct NodeId {
    pub id: ID,
    pub address: String,
}

pub struct Node {
    /// An instance of the blockchain held by this node
    pub chain: Arc<Mutex<Chain>>,

    /// Contains a unique identifier for this Node
    /// and it's associated Ip Address
    pub id: NodeId,

    /// Connected peers
    pub peers: HashSet<Arc<RwLock<dyn Peer>>>,

    /// A set of unconfirmed records held by this Node
    pub mem_pool: Arc<RwLock<dyn MemPool>>,

    /// A map of confirmed and published records cast and signed by each user
    /// Only records between members of this Node are kept within this node
    pub transactions: Arc<Mutex<dyn NodeRecord>>,

    /// A copy of the blockchain containing records that
    /// are relevant to peers in this Node
    ///
    pub local_chain: Chain,

    /// Network of nodes connected to this node
    ///
    /// A network can be for diverse purposes
    pub network: Arc<Mutex<Vec<NodeId>>>,

    pub miners: Arc<RwLock<Vec<Box<dyn Miner>>>>,

    pub units: crate::axs::unit::UnitManager,
}

impl Node {
    pub async fn broadcast<R: Record>(&self, block: SignedRecord<R>) {
        todo!()
    }

    pub fn poll_mem_pool<R: Record>(&self) -> Option<SignedRecord<R>>{
        todo!()
    }
    
}
