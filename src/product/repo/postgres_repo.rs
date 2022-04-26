use crate::product::json::{product::Product, table::Products};

use super::ProductRepo;
use crate::pb::{CreateProductRequest, ListProductRequest, UpdateProductRequest};
use async_trait::async_trait;
use common::utils::alias::{AppResult, PostgresAcquire};
use common::utils::error::AppError;
use sea_query::{Expr, PostgresQueryBuilder, Query};

pub struct ProductRepoImpl;

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Option<Product>> {
        let mut conn = executor.acquire().await.unwrap();

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
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        dbg!(&product);

        product
    }

    async fn create(
        &self,
        request: CreateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Product> {
        let mut conn = executor.acquire().await.unwrap();

        let name = request.name.clone().into();
        let currency = request.currency.into();
        let price = request.price.into();
        let now = chrono::Utc::now().into();

        let cols: Vec<Products> = vec![
            Products::Id,
            Products::Name,
            Products::Currency,
            Products::Price,
            Products::CreatedAt,
        ];

        let sql = Query::insert()
            .into_table(Products::Table)
            .columns(cols.clone())
            .values_panic(vec!["1".into(), name, currency, price, now])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        sqlx::query_as::<_, Product>(&sql)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn update(
        &self,
        request: UpdateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<bool> {
        let mut conn = executor.acquire().await.unwrap();

        let mut update_values = vec![];
        if let Some(name) = request.name {
            update_values.push((Products::Name, name.into()));
        }

        if let Some(currency) = request.currency {
            update_values.push((Products::Currency, currency.into()));
        }

        if let Some(price) = request.price {
            update_values.push((Products::Price, price.into()));
        }

        let sql = Query::update()
            .table(Products::Table)
            .values(update_values)
            .and_where(Expr::col(Products::Id).eq(request.id))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        sqlx::query(&sql)
            .execute(&mut *conn)
            .await
            .map(|e| e.rows_affected() > 0)
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn list(
        &self,
        request: ListProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Vec<Product>> {
        let mut conn = executor.acquire().await.unwrap();

        let cursor = request.cursor;
        let query = request.query.map(|q| format!("%{}%", q));
        let page_size = request.page_size;

        let sql = Query::select()
            .columns(vec![
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
            ])
            .and_where_option(query.map(|e| Expr::col(Products::Name).like(&e)))
            .and_where_option(cursor.map(|e| Expr::col(Products::Id).eq(e)))
            .from(Products::Table)
            .limit(page_size as u64)
            .to_string(PostgresQueryBuilder);

        sqlx::query_as::<_, Product>(&sql)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
