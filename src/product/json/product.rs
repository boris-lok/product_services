use chrono::{DateTime, Utc};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::pb;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub currency: i16,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Product> for pb::Product {
    fn from(p: Product) -> Self {
        Self {
            id: p.id as u64,
            name: p.name,
            currency: p.currency as i32,
            price: p.price.to_f64().unwrap(),
            created_at: p.created_at.timestamp_millis() as u64,
            updated_at: p.updated_at.map(|e| e.timestamp_millis() as u64),
            deleted_at: p.deleted_at.map(|e| e.timestamp_millis() as u64),
        }
    }
}
