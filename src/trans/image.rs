use super::record::SignedRecord;

pub trait ChainImage<X> {
    type BlockType: BlockImage<X>;
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
