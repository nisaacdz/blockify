use crate::{
    block::{Block, UnchainedInstance},
    chain::{Chain, ChainError},
    record::{Record, SignedRecord},
    PublicKey,
};

pub enum NodeError {
    ChainError(ChainError),
    VerificationFailed,
    ConnectionFailed,
}

pub trait MemPool<R: Record> {
    fn records(&self) -> Result<Vec<SignedRecord<R>>, MemPoolError>;
    fn poll(&mut self) -> Result<Option<SignedRecord<R>>, MemPoolError>;
    fn append(&mut self, record: SignedRecord<R>) -> Result<(), MemPoolError>;
}

pub enum MemPoolError {}

pub trait Node: Sized {
    type RecordType: Record;
    type BlockType: Block<RecordType = Self::RecordType>;
    type ChainType: Chain<RecordType = Self::RecordType, BlockType = Self::BlockType>;
    type MemPoolType: MemPool<Self::RecordType>;
    type PeerType: Peer;
    type NodeIdType: NodeId<Self>;

    fn publish(&mut self, record: SignedRecord<Self::RecordType>) -> Result<Feedback, NodeError>;
    fn chain(&self) -> Result<Self::ChainType, NodeError>;
    fn broadcast(&self, block: Self::BlockType) -> Result<Feedback, NodeError>;
    fn mem_pool(&self) -> Result<Option<Self::MemPoolType>, NodeError>;
    fn push(
        &mut self,
        block: &UnchainedInstance<Self::RecordType>,
    ) -> Result<<Self::ChainType as Chain>::ChainedInstanceType, NodeError> {
        self.chain()?
            .append(block)
            .map_err(|e| NodeError::ChainError(e))
    }

    fn peers(&self) -> Result<Vec<Self::PeerType>, NodeError>;
    fn network(&self) -> Result<Vec<Self::NodeIdType>, NodeError>;
}

pub trait NodeId<N: Node> {
    fn load(self) -> Result<N, NodeError>;
}

pub enum Feedback {}

pub trait Peer {
    fn public_key(&self) -> &PublicKey;
}
