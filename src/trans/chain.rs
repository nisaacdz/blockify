use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use crate::{
    errs::*,
    io::BlockBase,
    net::Pusher,
    ver::vers::{BlockVerifier, VerificationResult},
};

use super::{
    blocks::{Block, BlockBuilder},
    record::{Record, SignedRecord},
};

pub struct ChainBase {}

impl ChainBase {
    pub fn get_base<X: Record>(
        &self,
    ) -> Option<Arc<Mutex<dyn BlockBase<BlockBuilder<X>, SignedRecord<X>, X>>>> {
        todo!()
    }
}

pub struct Chain {
    cb: ChainBase,
}

impl Chain {
    pub fn push<'a, X: Record, P: Pusher<V>, V: BlockVerifier>(
        &self,
        data: BlockBuilder<X>,
        pusher: &P,
    ) -> Result<Block<X>, ChainBaseErrs<X>> {
        if !Self::verify_block(&data, pusher.get_verifier()).allow() {
            return Err(ChainBaseErrs::VerificationFailed);
        }

        match self.cb.get_base::<X>() {
            Some(bb) => match bb.lock() {
                Ok(mut val) => match val.borrow_mut().insert(data) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ChainBaseErrs::FromBlockBaseErrs(e)),
                },
                Err(_) => Err(ChainBaseErrs::PoisonedMutex),
            },
            None => Err(ChainBaseErrs::NoSuchChain),
        }
    }

    fn verify_block<X: Record, V: BlockVerifier>(
        block: &BlockBuilder<X>,
        verifier: Arc<Mutex<V>>,
    ) -> VerificationResult {
        todo!()
    }
}
