use crate::trans::record::SignedRecord;


pub enum NodeError {}


pub trait Node {
    fn publish<R>(&self, record: SignedRecord<R>) -> Result<(), NodeError>;
    fn pool<R>(&self, record: SignedRecord<R>) -> Result<(), NodeError>;
}
