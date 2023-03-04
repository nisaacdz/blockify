use std::sync::{Arc, Mutex};

use crate::io::ChainBase;

pub struct Chain {
    _db: Arc<Mutex<dyn ChainBase>>,
}
