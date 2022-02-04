use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct PostgresRepository {
    connection_pool: Arc<Pool<Postgres>>
}

impl PostgresRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { connection_pool: pool }
    }
}
