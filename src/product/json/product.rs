use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub currency: i16,
    pub price: Decimal,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
