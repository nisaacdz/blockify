use crate::trans::record::SignedRecord;

pub struct NodeId {}

pub struct NodeConnection {}

pub trait Node {
    fn connect(&self) -> NodeConnection;
    fn publish<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
    fn investigate<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
}
