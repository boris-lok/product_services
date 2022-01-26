use crate::core::config::config::Config;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Env {
    pub config: Arc<Config>,
}

impl Env {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}
