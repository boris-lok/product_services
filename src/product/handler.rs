use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::pb::{
    product_services_server::ProductServices, CreateProductRequest, GetProductRequest,
    GetProductResponse, ListProductRequest, ListProductResponse, Product, UpdateProductRequest,
};
use crate::product::services::{ProductService, ProductServiceImpl};

pub struct ProductServicsImpl {
    session: Pool<Postgres>,
}

impl ProductServicsImpl {
    pub fn new(session: Pool<Postgres>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl ProductServices for ProductServicsImpl {
    async fn create(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        let product = services.create(request).await.map(|e| e.into());

        if product.is_err() {
            return Err(Status::failed_precondition("failed to create a product."));
        }

        Ok(Response::new(product.unwrap()))
    }

    async fn update(
        &self,
        request: Request<UpdateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        let product = services.update(request).await.map(|e| e.into());

        if product.is_err() {
            return Err(Status::failed_precondition("failed to update a product."));
        }

        Ok(Response::new(product.unwrap()))
    }

    async fn get(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<GetProductResponse>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        let product = services
            .get(request.id as i64)
            .await
            .map(|e| e.map(|p| p.into()));

        if product.is_err() {
            return Err(Status::failed_precondition("failed to get a product."));
        }

        Ok(Response::new(GetProductResponse {
            product: product.unwrap(),
        }))
    }

    async fn list(
        &self,
        request: Request<ListProductRequest>,
    ) -> Result<Response<ListProductResponse>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        let products = services.list(request).await.map(|e| {
            let e = e.into_iter().map(|p| p.into()).collect::<_>();

            ListProductResponse { products: e }
        });

        if products.is_err() {
            return Err(Status::failed_precondition("failed to list products."));
        }

        Ok(Response::new(products.unwrap()))
    }
}
