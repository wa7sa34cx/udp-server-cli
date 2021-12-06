use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Request error: {0}")]
    ReqError(String),

    #[error("Database error: {0}")]
    DbError(String),
}
