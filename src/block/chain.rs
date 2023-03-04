use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use crate::{
    errs::*,
    io::BlockBase,
    record::{Record, SignedRecord},
};

use super::{Block, BlockBuilder};

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
    pub fn push<'a, X: Record>(&self, data: BlockBuilder<X>) -> Result<Block<X>, ChainBaseErrs<X>> {
        self.verify_block(&data)?;

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

    fn verify_block<X: Record>(&self, block: &BlockBuilder<X>) -> Result<(), ChainBaseErrs<X>> {
        match block.verify() {
            Ok(()) => Ok(()),
            Err(e) => match e {
                Errs::InvalidRecord(v) => Err(ChainBaseErrs::InvalidRecordInBlock(v.clone())),
                _ => Err(ChainBaseErrs::UnknownErrs), // Will never occur,
            },
        }
    }
}
