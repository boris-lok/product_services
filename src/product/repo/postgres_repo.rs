use crate::product::json::product::Product;

use super::ProductRepo;
use crate::pb::{
    CreateProductRequest, GetProductRequest, ListProductRequest, UpdateProductRequest,
};
use async_trait::async_trait;
use common::utils::alias::{AppResult, PostgresAcquire};

pub struct ProductRepoImpl;

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Option<Product>> {
        todo!()
    }

    async fn create(
        &self,
        request: CreateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Product> {
        todo!()
    }

    async fn update(
        &self,
        request: UpdateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<bool> {
        todo!()
    }

    async fn list(
        &self,
        request: ListProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Vec<Product>> {
        todo!()
    }
}
