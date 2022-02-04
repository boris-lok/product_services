use async_trait::async_trait;
use mockall::automock;

use crate::{core::alias_type::AppResult, product::json::product::Product};

#[automock]
#[async_trait]
pub(crate) trait ProductRepository {
    async fn get(&self, id:i64) -> AppResult<Option<Product>>;

    async fn list(&self, keyword: Option<String>, updated_at: Option<chrono::DateTime<chrono::Utc>>, page_size: usize) -> AppResult<Vec<Product>>;

    async fn create(&self, name: &str, currency_code: u8, price: i32) -> AppResult<Product>;
}
