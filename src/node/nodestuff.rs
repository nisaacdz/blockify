#[cfg(feature = "blockchain")]
use crate::trans::record::SignedRecord;

#[cfg(feature = "blockchain")]
pub enum NodeError {}

#[cfg(feature = "blockchain")]
pub trait Node {
    fn publish<R>(&self, record: SignedRecord<R>) -> Result<(), NodeError>;
    fn pool<R>(&self, record: SignedRecord<R>) -> Result<(), NodeError>;
}
