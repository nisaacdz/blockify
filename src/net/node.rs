use crate::{
    axs::unit::Units,
    trans::{
        blocks::{Block, BlockBuilder},
        record::{Record, SignedRecord},
    },
};

use super::{Miner, Peer};

#[derive(Clone, Debug)]
pub struct NodeId {
    pub id: crate::dat::ID,
    pub address: String,
}

pub enum NodeErrors {}

pub trait Obtainer {
    type Item;
    fn obtain(&self) -> Self::Item;
}

pub trait Node {
    fn units(&self) -> Units;
    fn miners(&self) -> Vec<Box<dyn Miner>>;
    fn network(&self) -> Vec<NodeId>;
    fn id(&self) -> NodeId;
    fn peers(&self) -> Vec<Box<dyn Peer>>;
    fn mem_pool<R: Record>(
        &self,
    ) -> Box<dyn Obtainer<Item = Box<dyn Iterator<Item = BlockBuilder<R>>>>>;
    fn pending<R: Record>(
        &self,
    ) -> Box<dyn Obtainer<Item = Box<dyn Iterator<Item = BlockBuilder<R>>>>>;
    fn chain(&self) -> Box<dyn Obtainer<Item = Box<dyn Iterator<Item = Block>>>>;
    fn local_chain(&self) -> Box<dyn Obtainer<Item = Box<dyn Iterator<Item = Block>>>>;
    // fn records(&self) -> Box<dyn NodeRecord>;

    fn publish(&self) -> Result<(), NodeErrors>;

    fn poll_mem_pool<R: Record>(&self) -> Result<BlockBuilder<R>, NodeErrors>;
    fn broadcast<R: Record>(&self) -> Result<(), NodeErrors>;
    fn push_to_pending<R: Record>(&mut self, record: SignedRecord<R>) -> Result<(), NodeErrors>;
    fn push_to_mem_pool<R: Record>(&mut self, record: SignedRecord<R>) -> Result<(), NodeErrors>;
}
