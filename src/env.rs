use crate::{core::config::config::Config, product::repo::repository::ProductRepository};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Env {
    pub config: Arc<Config>,
    pub repo: Arc<ProductRepository>,
}

impl Env {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}
