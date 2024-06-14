use crate::project::ProjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectStorageError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Project already exists: {}", .0)]
    ProjectAlreadyExists(ProjectId),
    #[error("Project creation failed")]
    ProjectCreationFailed,
    #[error("Project update failed")]
    ProjectUpdateFailed,
    #[error("Project delete failed")]
    ProjectDeleteFailed,
    #[error("Failed to update files store. {}", .0)]
    ProjectFailedToSave(FileSystemStorageError),
    #[error("Failed to load files store. {}", .0)]
    ProjectFailedToLoad(FileSystemStorageError),
}

#[derive(Debug, Error)]
pub enum FileSystemStorageError {
    #[error("Could not load file: {}", .0)]
    FileLoadError(String),
    #[error("Could not save file: {}", .0)]
    FileSaveError(String),
}
