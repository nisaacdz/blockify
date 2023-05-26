mod nodestuff;

use crate::{
    block::{Block, UnchainedInstance},
    chain::{Chain, ChainError},
    record::{Record, SignedRecord},
    PublicKey,
};

pub enum NodeError {
    ChainError(ChainError),
    VerificationFailed,
}

pub trait MemPool {
    type RecordType: Record;
    fn records(&self) -> Result<Vec<SignedRecord<Self::RecordType>>, MemPoolError>;
    fn poll(&mut self) -> Result<Option<SignedRecord<Self::RecordType>>, MemPoolError>;
    fn append(&mut self, record: SignedRecord<Self::RecordType>) -> Result<(), MemPoolError>;
}

pub enum MemPoolError {}

pub trait Node: Sized {
    type RecordType: Record;
    type BlockType: Block<RecordType = Self::RecordType>;
    type ChainType: Chain<RecordType = Self::RecordType, BlockType = Self::BlockType>;
    type MemPoolType: MemPool<RecordType = Self::RecordType>;
    type PeerType: Peer;
    type NodeIdType: NodeId<Self>;

    fn publish(
        &mut self,
        record: SignedRecord<<Self::ChainType as Chain>::RecordType>,
    ) -> Result<Feedback, NodeError>;
    fn chain(&self) -> Result<Self::ChainType, NodeError>;
    fn broadcast(
        &self,
        block: <Self::ChainType as Chain>::BlockType,
    ) -> Result<Feedback, NodeError>;
    fn mem_pool(&self) -> Result<Option<Self::MemPoolType>, NodeError>;
    fn push(
        &mut self,
        block: &UnchainedInstance<<Self::ChainType as Chain>::RecordType>,
        proof: impl MinerProof,
    ) -> Result<<Self::ChainType as Chain>::ChainInstanceType, NodeError> {
        if proof.verify() {
            self.chain()?
                .append(block)
                .map_err(|e| NodeError::ChainError(e))
        } else {
            Err(NodeError::VerificationFailed)
        }
    }

    fn peers(&self) -> Result<Vec<Self::PeerType>, NodeError>;
    fn network(&self) -> Result<Vec<Self::NodeIdType>, NodeError>;
}

pub trait NodeId<N: Node> {
    fn load(self) -> Result<N, NodeError>;
}

pub trait MinerProof {
    fn verify(&self) -> bool;
}

pub enum Feedback {}

pub trait Peer {
    fn public_key(&self) -> &PublicKey;
}
