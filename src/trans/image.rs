use super::{blocks::ChainedInstance, record::SignedRecord};

pub trait ChainImage {
    type ErrorType;
    type BlockType: AsRef<ChainedInstance>;
    fn cur(&mut self) -> Option<Self::BlockType>;
    fn next(&mut self) -> Option<Self::BlockType>;
    fn prev(&mut self) -> Option<Self::BlockType>;
}

pub trait BlockImage<X> {
    type RecordType: AsRef<SignedRecord<X>>;
    fn cur(&mut self) -> Option<Self::RecordType>;
    fn next(&mut self) -> Option<Self::RecordType>;
    fn prev(&mut self) -> Option<Self::RecordType>;
}
