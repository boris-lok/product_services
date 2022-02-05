use serde::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    DatabaseError(String),
}

impl warp::reject::Reject for AppError {}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl From<(u16, &str)> for ErrorResponse {
    fn from(e: (u16, &str)) -> Self {
        Self {
            code: e.0,
            message: e.1.to_string(),
        }
    }
}
