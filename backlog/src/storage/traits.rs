use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::prelude::*;
use crate::project::{DeletedProject, Project, ProjectDraft, ProjectId, ProjectPatch};

pub enum ProjectQuery {
    GetProject(ProjectId),
    GetProjects,
    CreateProject(Project),
    UpdateProject(ProjectId, Project),
    DeleteProject(ProjectId),
}
pub trait Filestore {
    fn new(path: impl Into<String>) -> Self;
    fn load<T: DeserializeOwned>(&self) -> Result<T, FileSystemStorageError>
    where
        Self: Sized;
    fn save<T: Serialize>(&self, to_save: T) -> Result<(), FileSystemStorageError>;
}

pub trait Database {
    fn connect(&self) -> bool;
    fn query(&self, query: &str) -> Vec<String>;
    fn save(&self, data: &str) -> bool;
}

pub trait Storage {
    fn query_one(&mut self, query: ProjectQuery) -> Result<Project, ProjectStorageError>;
    fn query_many(&mut self, query: ProjectQuery) -> Result<Vec<Project>, ProjectStorageError>;
    fn create(&mut self, query: ProjectQuery) -> Result<ProjectId, ProjectStorageError>;
    fn update(&mut self, query: ProjectQuery) -> Result<Project, ProjectStorageError>;
    fn delete(&mut self, query: ProjectQuery) -> Result<DeletedProject, ProjectStorageError>;

    // pub fn save(&self, to_save: S) -> Result<(), ProjectStorageError> {
    //     self.file_store
    //         .save(to_save)
    //         .map_err(ProjectStorageError::ProjectFailedToSave)
    // }
}

pub trait ProjectStorage {
    fn get_project(self, id: ProjectId) -> Result<Project, ProjectStorageError>;
    fn get_projects(self) -> Result<Vec<Project>, ProjectStorageError>;
    fn create_project(&mut self, draft: ProjectDraft) -> Result<ProjectId, ProjectStorageError>;
    fn update_project(
        &mut self,
        id: ProjectId,
        patch: ProjectPatch,
    ) -> Result<Project, ProjectStorageError>;
    fn delete_project(&mut self, id: ProjectId) -> Result<DeletedProject, ProjectStorageError>;
}
