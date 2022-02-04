use clap::Parser;

use crate::core::args::Args;
use crate::core::config::postgres_config::PostgresConfig;
use crate::core::config::redis_config::RedisConfig;

mod core;
mod env;
mod product;

fn main() {
    let args = Args::parse();

    let _ = dotenv::from_path(args.env_file);
    let postgres = PostgresConfig::new();
    let redis = RedisConfig::new();
}
