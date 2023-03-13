use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use crate::{errs::BlockifyError, io::BlockBase};

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
    pub fn append<'a, X: Record>(
        &self,
        data: BlockBuilder<X>,
    ) -> Result<Block<X>, BlockifyError> {
        match self.cb.get_base::<X>() {
            Some(bb) => match bb.lock() {
                Ok(mut val) => match val.borrow_mut().insert(data) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(BlockifyError::unknown()),
                },
                Err(_) => Err(BlockifyError::new("Poisoned Mutex")),
            },
            None => Err(BlockifyError::new("No Such Chain")),
        }
    }

    pub fn unroll(&self) -> bool {
        todo!()
    }
}
