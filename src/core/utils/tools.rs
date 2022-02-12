use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use r2d2_redis::r2d2;
use r2d2_redis::RedisConnectionManager;
use snowflake::SnowflakeGenerator;

use crate::core::config::postgres_config::PostgresConfig;
use crate::core::config::redis_config::RedisConfig;
use crate::core::config::id_generator_config::IdGeneratorConfig;
use crate::core::alias_type::AppResult;
use crate::core::error::AppError;


/// Create a database connection.
pub async fn create_database_connection(config: PostgresConfig) -> AppResult<Pool<Postgres>> {
   let connection_options = PgConnectOptions::new() 
       .host(&config.host)
       .database(&config.database)
       .username(&config.username)
       .password(&config.password)
       .port(config.port);

   PgPoolOptions::new()
       .max_connections(config.max_connection)
       .connect_with(connection_options)
       .await
       .map_err(|e| AppError::ConnectionError(e.to_string()))
}


/// Create a redis connection
pub async fn create_redis_connection(config: RedisConfig) -> AppResult<r2d2::Pool<RedisConnectionManager>> {
    let redis_uri = format!(
        "redis://{}:{}@{}:{}",
        config.username, config.password, config.host, config.port
        );

    let manager = RedisConnectionManager::new(redis_uri)
        .map_err(|e| AppError::ConnectionError(e.to_string()));

    if let Ok(manager) = manager {
        r2d2::Pool::builder()
            .build(manager)
            .map_err(|e| AppError::ConnectionError(e.to_string()))
    } else {
        Err(manager.unwrap_err())
    }
}


/// Create a id generator
pub fn create_id_generator(config: IdGeneratorConfig) -> SnowflakeGenerator {
    SnowflakeGenerator::new(config.worker_id, config.data_center_id, config.timestamp_offset)
}
