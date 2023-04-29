use super::{blocks::ChainedInstance, chain::ChainErrors};

pub trait ChainImage {
    fn cur(&mut self) -> Result<Option<ChainedInstance>, ChainErrors>;
    fn next(&mut self) -> Result<Option<ChainedInstance>, ChainErrors>;
    fn prev(&mut self) -> Result<Option<ChainedInstance>, ChainErrors>;
}
