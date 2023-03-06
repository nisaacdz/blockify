use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    trans::{chain::Chain, record::SignedRecord},
    UnitManager,
};

use super::{Entity, Pusher, Record};

#[derive(Clone, Debug)]
pub struct NodeId {
    pub id: u128,
    pub address: String,
}

pub struct Node<R: Record> {
    /// An instance of the blockchain held by this node
    pub chain: Arc<Mutex<Chain>>,

    /// Contains a unique identifier for this Node
    /// and it's associated Ip Address
    pub id: NodeId,

    /// Connected peers
    pub peers: HashSet<Box<dyn Entity<R>>>,

    /// A set of unconfirmed records held by this Node
    pub mem_pool: Arc<Mutex<Vec<SignedRecord<R>>>>,

    /// A map of confirmed and published records cast and signed by each user
    /// Only records between members of this Node are kept within this node
    pub transactions: Arc<Mutex<HashMap<u32, HashSet<R>>>>,

    /// A copy of the blockchain containing records that
    /// are relevant to peers in this Node
    ///
    pub local_chain: Chain,

    /// Network of nodes connected to this node
    ///
    /// A network can be for diverse purposes
    pub network: Arc<Mutex<Vec<NodeId>>>,

    pub miners: Arc<RwLock<Vec<Box<dyn Pusher>>>>,

    pub units: UnitManager,
}
