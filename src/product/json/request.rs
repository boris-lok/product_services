use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub currency: u8,
    pub price: isize,
}
