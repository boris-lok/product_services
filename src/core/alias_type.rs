use super::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
pub type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;
