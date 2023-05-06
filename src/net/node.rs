use crate::{axs::dat::ID, trans::record::SignedRecord};

pub struct NodeId {
    _id: ID,
    _ip: String,
}

pub trait NodeConnection {}

pub trait Node {
    fn connect<Con: NodeConnection>(&self) -> Con;
    fn publish<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
    fn investigate<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
}
