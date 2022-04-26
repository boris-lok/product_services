pub mod postgres_repo;

use super::json::product::Product;
use crate::pb::{CreateProductRequest, ListProductRequest, UpdateProductRequest};
use async_trait::async_trait;
use common::utils::alias::{AppResult, PostgresAcquire};

#[async_trait]
pub trait ProductRepo {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Option<Product>>;

    async fn create(
        &self,
        request: CreateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Product>;

    async fn update(
        &self,
        request: UpdateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<bool>;

    async fn list(
        &self,
        request: ListProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Vec<Product>>;
}
