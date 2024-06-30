use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateProjectError {
    #[error("Project already exists")]
    ProjectAlreadyExists,
    #[error("Unknown data error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum GetAllProjectsError {
    #[error("Unknown data error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum GetOneProjectError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Unknown data error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum UpdateProjectError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Unknown data error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Failed to connect to database")]
    ConnectionFailed,
    #[error("Transaction failed")]
    TransactionFailed,
    #[error("Migration failed")]
    MigrationFailed,
}
