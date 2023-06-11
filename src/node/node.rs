use crate::{
    block::{Block, PositionInstance, UnchainedInstance},
    chain::{Chain, ChainError},
    record::{Record, SignedRecord},
    PublicKey, DigitalSignature,
};

pub enum NodeError {
    ChainError(ChainError),
    Unimplemented,
    VerificationFailed,
    ConnectionFailed,
}

pub trait MemPool<R: Record> {
    fn records(&self) -> Result<Vec<SignedRecord<R>>, MemPoolError>;
    fn poll(&mut self) -> Result<Option<SignedRecord<R>>, MemPoolError>;
    fn append(&mut self, record: SignedRecord<R>) -> Result<(), MemPoolError>;
}

pub enum MemPoolError {}

pub trait Node<R: Record>: Sized {
    type UnchainedInstanceType: UnchainedInstance<R>;
    type BlockType: Block<R>;
    type ChainType: Chain<
        R,
        BlockType = Self::BlockType,
        UnchainedInstanceType = Self::UnchainedInstanceType,
    >;
    type MemPoolType: MemPool<R>;
    type PeerType: Peer<R>;
    type NodeIdType: NodeId;

    fn publish(&mut self, record: SignedRecord<R>) -> Result<Feedback, NodeError>;
    fn chain(&self) -> Result<Self::ChainType, NodeError>;
    fn broadcast(&self, block: Self::BlockType) -> Result<Feedback, NodeError>;
    fn mem_pool(&self) -> Result<Option<Self::MemPoolType>, NodeError>;
    fn push(&mut self, block: Self::UnchainedInstanceType) -> Result<PositionInstance, NodeError> {
        self.chain()?
            .append(&block)
            .map_err(|e| NodeError::ChainError(e))
    }

    fn peers(&self) -> Result<Vec<Self::PeerType>, NodeError>;
    fn network(&self) -> Result<Vec<Self::NodeIdType>, NodeError>;
}

pub trait NodeId {
    type NodeType;
    fn load(self) -> Result<Self::NodeType, NodeError>;
}

pub enum Feedback {}

pub trait Peer<R: Record> {
    fn public_key(&self) -> &PublicKey;
    fn sign(&self, _: R) -> Result<DigitalSignature, PeerError> {
        todo!()
    }
    fn verify(&self, signature: &DigitalSignature, record: R) -> bool {
        record.verify(signature, self.public_key()).is_ok()
    }
}
pub enum PeerError {}
pub enum MiningError {}

pub trait Miner<R: Record> {
    fn append(&self, record: SignedRecord<R>) -> Result<(), MiningError>;
}
