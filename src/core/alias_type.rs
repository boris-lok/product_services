pub type AppResult<T> = anyhow::Result<T>;
pub type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;
