use thiserror::Error;

#[derive(Error, Debug)]
pub enum LcError {
    #[error("LeetCode API Error: {0}")]
    ApiError(String),
    #[error("Problem not found: {0}")]
    ProblemNotFound(String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Other: {0}")]
    Other(#[from] anyhow::Error),
}
