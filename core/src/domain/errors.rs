use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsertProjectError {
    #[error("Project already exists")]
    ProjectAlreadyExists,
    #[error("Unknown data error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ListProjectError {
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
