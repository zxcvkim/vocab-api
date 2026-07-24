use std::sync::Arc;

use crate::{Vocab, config::Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub vocab: Arc<Vocab>,
}

impl AppState {
    pub fn new(config: Config, vocab: Vocab) -> Self {
        Self {
            config: Arc::new(config),
            vocab: Arc::new(vocab),
        }
    }
}
