use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use snowflake::SnowflakeGenerator;
use tonic::transport::Server;

use common::{
    configs::id_generator_config::IdGeneratorConfig, configs::postgres_config::PostgresConfig,
    utils::tools::create_database_connection, utils::tools::create_id_generator,
};
use common::configs::config::Config;
use common::utils::alias::AppResult;
use common::utils::tools::tracing_initialize;
use pb::product_services_server::ProductServicesServer;
use product::handler::ProductServicesImpl;

mod product;

mod pb {
    include!("../gen/grpc.product.rs");
}

lazy_static! {
    static ref ID_GENERATOR: Arc<Mutex<SnowflakeGenerator>> = {
        let config = IdGeneratorConfig::new();
        let generator = create_id_generator(config);
        Arc::new(Mutex::new(generator))
    };
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let _ = dotenv::from_path("env/dev.env").unwrap();

    let config = Config::new();

    tracing_initialize(config.debug, "logs/", "products");

    let database_config = PostgresConfig::new();

    let database_connection = create_database_connection(database_config).await.unwrap();

    let service = ProductServicesImpl::new(database_connection);

    let addr = "[::1]:50002".parse().unwrap();

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .add_service(ProductServicesServer::new(service))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
