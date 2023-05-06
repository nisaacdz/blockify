use crate::io::DataBaseError;

use super::record::SignedRecord;

pub trait ChainImage<X> {
    type BlockType: BlockImage<X>;
    fn cur(&mut self) -> Result<Option<Self::BlockType>, DataBaseError>;
    fn next(&mut self) -> Result<Option<Self::BlockType>, DataBaseError>;
    fn prev(&mut self) -> Result<Option<Self::BlockType>, DataBaseError>;
}

pub trait BlockImage<X> {
    type RecordType: AsRef<SignedRecord<X>>;
    fn cur(&mut self) -> Result<Option<Self::RecordType>, DataBaseError>;
    fn next(&mut self) -> Result<Option<Self::RecordType>, DataBaseError>;
    fn prev(&mut self) -> Result<Option<Self::RecordType>, DataBaseError>;
}
