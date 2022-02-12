use clap::Parser;

use crate::core::args::Args;
use crate::core::config::postgres_config::PostgresConfig;
use crate::core::config::redis_config::RedisConfig;
use crate::core::config::id_generator_config::IdGeneratorConfig;
use crate::core::utils::tools::create_database_connection;
use crate::core::utils::tools::create_redis_connection;
use crate::core::utils::tools::create_id_generator;

mod core;
mod env;
mod product;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let _ = dotenv::from_path(args.env_file);
    let postgres_config = PostgresConfig::new();
    let redis_config = RedisConfig::new();
    let id_generator_config = IdGeneratorConfig::new();

    let database_connection = create_database_connection(postgres_config)
        .await
        .expect("Can connect to database");
    let redis_connection = create_redis_connection(redis_config)
        .await
        .expect("Can connect to redis");
}

