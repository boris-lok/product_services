use crate::core::alias_type::AppResult;
use r2d2_redis::{r2d2, RedisConnectionManager};

#[derive(Debug)]
pub struct RedisConfig {
    host: String,
    username: String,
    password: String,
    port: u16,
}

impl RedisConfig {
    pub fn new() -> Self {
        let host = dotenv::var("REDIS_HOST").expect("Can read the redis host from env.");

        let username = dotenv::var("REDIS_USERNAME").unwrap_or_else(|_| "".to_owned());
        let password = dotenv::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_owned());
        let port = dotenv::var("REDIS_PORT")
            .expect("Can read the redis port")
            .parse::<u16>()
            .expect("Can parse the port to u16");

        Self {
            host,
            username,
            password,
            port,
        }
    }

    pub async fn create_connection(&self) -> AppResult<r2d2::Pool<RedisConnectionManager>> {
        let redis_uri = format!(
            "redis://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        );

        let manager = RedisConnectionManager::new(redis_uri)?;
        Ok(r2d2::Pool::builder().build(manager)?)
    }
}
