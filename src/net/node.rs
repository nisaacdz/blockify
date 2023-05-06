use crate::trans::record::SignedRecord;

pub struct NodeId {}

pub trait NodeConnection {
    
}

pub trait Node {
    fn connect<Con: NodeConnection>(&self) -> Con;
    fn publish<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
    fn investigate<R>(&self, record: SignedRecord<R>) -> Result<(), ()>;
    
}
