use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BacklogError {
    #[error("Failed to load backlog config: {0}")]
    BacklogConfigLoadError(String),
    #[error("Failed to save backlog config: {0}")]
    BacklogConfigSaveError(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
