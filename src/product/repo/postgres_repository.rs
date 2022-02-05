use async_trait::async_trait;
use std::sync::Arc;

use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

use crate::{
    core::{alias_type::AppResult, error::AppError},
    product::json::{product::Product, table::Products},
};

use super::repository::ProductRepository;

#[derive(Clone)]
pub struct PostgresRepository {
    connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            connection_pool: pool,
        }
    }
}

#[async_trait]
impl ProductRepository for PostgresRepository {
    async fn get(&self, id: i64) -> AppResult<Option<Product>> {
        let sql = Query::select()
            .columns([
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
                Products::DeletedAt,
            ])
            .from(Products::Table)
            .and_where(Expr::col(Products::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let product = sqlx::query_as::<_, Product>(sql.as_str())
            .fetch_optional(&*self.connection_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        dbg!(&product);

        product
    }

    async fn create(
        &self,
        id: i64,
        name: &str,
        currency_code: u8,
        price: i32,
    ) -> AppResult<Product> {
        let sql = Query::insert()
            .into_table(Products::Table)
            .columns([
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
            ])
            .values_panic(vec![
                id.into(),
                name.into(),
                currency_code.into(),
                price.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .to_owned()
            .returning(
                Query::select()
                    .columns(vec![
                        Products::Id,
                        Products::Name,
                        Products::Currency,
                        Products::Price,
                        Products::CreatedAt,
                        Products::UpdatedAt,
                        Products::DeletedAt,
                    ])
                    .take(),
            )
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let product = sqlx::query_as::<_, Product>(sql.as_str())
            .fetch_one(&*self.connection_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        dbg!(&product);

        product
    }

    async fn list(
        &self,
        keyword: Option<String>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        page_size: usize,
    ) -> AppResult<Vec<Product>> {
        let sql = Query::select()
            .columns(vec![
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
                Products::DeletedAt,
            ])
            .and_where_option(
                keyword.map(|e| Expr::col(Products::Name).like(format!("%{}%", e).as_str())),
            )
            .and_where_option(updated_at.map(|e| Expr::col(Products::UpdatedAt).gt(e)))
            .from(Products::Table)
            .limit(page_size as u64)
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let products = sqlx::query_as::<_, Product>(sql.as_str())
            .fetch_all(&*self.connection_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        dbg!(&products);

        products
    }
}
