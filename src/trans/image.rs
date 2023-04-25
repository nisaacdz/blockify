use super::{blocks::ChainedBlock, chain::ChainErrors};

pub trait ChainImage {
    fn cur(&mut self) -> Result<Option<ChainedBlock>, ChainErrors>;
    fn next(&mut self) -> Result<Option<ChainedBlock>, ChainErrors>;
    fn prev(&mut self) -> Result<Option<ChainedBlock>, ChainErrors>;
}
