use common::utils::alias::AppResult;
use common::{configs::postgres_config::PostgresConfig, utils::tools::create_database_connection};
use pb::product_services_server::ProductServicesServer;
use product::handler::ProductServicsImpl;
use tonic::transport::Server;

mod product;

mod pb {
    include!("../gen/grpc.product.rs");
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::from_path("env/dev.env");

    let postgres = PostgresConfig::new();

    let database_connection = create_database_connection(postgres).await.unwrap();

    let service = ProductServicsImpl::new(database_connection);

    let addr = "127.0.0.1:50002".parse().unwrap();

    Server::builder()
        .add_service(ProductServicesServer::new(service))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
