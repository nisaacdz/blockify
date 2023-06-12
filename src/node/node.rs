use crate::{
    block::{ChainedInstance, PositionInstance, UnchainedInstance},
    chain::{Chain, ChainError},
    data::Metadata,
    record::{Record, SignedRecord},
    AuthKeyPair, DigitalSignature, PublicKey, SigningError,
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
    type ChainedInstanceType: ChainedInstance<R>;
    type ChainType: Chain<
        R,
        ChainedInstanceType = Self::ChainedInstanceType,
        UnchainedInstanceType = Self::UnchainedInstanceType,
    >;
    type MemPoolType: MemPool<R>;
    type PeerType: Peer<R>;
    type NodeIdType: NodeId<R>;

    fn publish(&mut self, record: SignedRecord<R>) -> Result<Feedback, NodeError>;
    fn chain(&self) -> Result<Self::ChainType, NodeError>;
    fn broadcast(&self, block: Self::ChainedInstanceType) -> Result<Feedback, NodeError>;
    fn mem_pool(&self) -> Result<Option<Self::MemPoolType>, NodeError>;
    fn push(&mut self, block: Self::UnchainedInstanceType) -> Result<PositionInstance, NodeError> {
        self.chain()?
            .append(&block)
            .map_err(|e| NodeError::ChainError(e))
    }

    fn peers(&self) -> Result<Vec<Self::PeerType>, NodeError>;
    fn network(&self) -> Result<Vec<Self::NodeIdType>, NodeError>;
}

pub trait NodeId<N> {
    fn load(self) -> Result<N, NodeError>;
}

pub enum Feedback {}

pub trait Peer<R: Record> {
    fn public_key(&self) -> &PublicKey;
    fn sign(record: &R, keypair: &AuthKeyPair) -> Result<DigitalSignature, SigningError> {
        record.sign(keypair)
    }

    fn record(
        record: R,
        keypair: AuthKeyPair,
        metadata: Metadata,
    ) -> Result<SignedRecord<R>, SigningError> {
        record.record(keypair, metadata)
    }

    fn verify(&self, signature: &DigitalSignature, record: R) -> bool {
        record.verify(signature, self.public_key()).is_ok()
    }
}

pub enum MiningError {}

pub trait Miner<R: Record> {
    fn append(&self, record: SignedRecord<R>) -> Result<(), MiningError>;
}
