use crate::{
    core::alias_type::{AppResult, WebResult},
    env::Env,
    product::{
        json::{product::Product, request::CreateProductRequest},
        repo::repository::ProductRepository,
    },
};

async fn create_product(
    repo: &dyn ProductRepository,
    env: &Env,
    request: CreateProductRequest,
) -> AppResult<Product> {
    let product = repo
        .create(
            0,
            request.name.as_str(),
            request.currency,
            request.price as i32,
        )
        .await;

    product
}
