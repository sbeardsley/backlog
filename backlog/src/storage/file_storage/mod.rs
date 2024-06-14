pub mod yaml;
use serde::Serialize;

use crate::project::{DeletedProject, Project, ProjectBuilder, ProjectId};
use crate::storage::error::ProjectStorageError;
use std::collections::HashMap;

use super::traits::{Filestore, ProjectQuery, Storage};

#[derive(Debug)]
pub struct FileStorage<T>
where
    T: Filestore,
{
    projects: HashMap<ProjectId, Project>,
    current_project_id: u32,
    file_store: T,
}

impl<T> FileStorage<T>
where
    T: Filestore,
{
    pub fn new(file_store: T) -> Self {
        Self {
            projects: HashMap::new(),
            current_project_id: 0,
            file_store,
        }
    }

    pub fn save<S: Serialize>(&self, to_save: S) -> Result<(), ProjectStorageError> {
        self.file_store
            .save(to_save)
            .map_err(ProjectStorageError::ProjectFailedToSave)
    }

    pub fn generate_id(&mut self) -> ProjectId {
        self.current_project_id += 1;
        self.current_project_id
    }
}

impl<T> Storage for FileStorage<T>
where
    T: Filestore,
{
    fn query_one(&mut self, query: ProjectQuery) -> Result<Project, ProjectStorageError> {
        match query {
            ProjectQuery::GetProject(id) => self
                .projects
                .get(&id)
                .cloned()
                .ok_or(ProjectStorageError::ProjectNotFound),
            _ => Err(ProjectStorageError::ProjectNotFound),
        }
    }

    fn query_many(&mut self, query: ProjectQuery) -> Result<Vec<Project>, ProjectStorageError> {
        match query {
            ProjectQuery::GetProjects => {
                Ok(self.projects.values().cloned().collect::<Vec<Project>>())
            }
            _ => Err(ProjectStorageError::ProjectNotFound),
        }
    }

    fn create(&mut self, query: ProjectQuery) -> Result<ProjectId, ProjectStorageError> {
        let id = self.generate_id();
        match query {
            ProjectQuery::CreateProject(draft) => {
                let new_project = ProjectBuilder::from_project(draft)
                    .id(id)
                    .build()
                    .map_err(|_| ProjectStorageError::ProjectCreationFailed)?;

                self.projects.insert(id, new_project);
                Ok(id)
            }
            _ => Err(ProjectStorageError::ProjectCreationFailed),
        }
    }

    fn update(&mut self, query: ProjectQuery) -> Result<Project, ProjectStorageError> {
        match query {
            ProjectQuery::UpdateProject(id, patch) => self
                .projects
                .insert(id, patch.clone())
                .ok_or(ProjectStorageError::ProjectUpdateFailed),
            _ => Err(ProjectStorageError::ProjectUpdateFailed),
        }
    }

    fn delete(&mut self, query: ProjectQuery) -> Result<DeletedProject, ProjectStorageError> {
        match query {
            ProjectQuery::DeleteProject(id) => self
                .projects
                .remove(&id)
                .map(DeletedProject::new)
                .ok_or(ProjectStorageError::ProjectDeleteFailed),
            _ => Err(ProjectStorageError::ProjectDeleteFailed),
        }
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use super::yaml::Yaml;
    use crate::storage::{file_storage::FileStorage, traits::Filestore};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        name: String,
    }

    #[test]
    fn test_can_construct_file_store_t() {
        let yaml_format = Yaml::new("~/.backlog/tests/projects.yaml");
        let test_struct = yaml_format.load::<TestStruct>().unwrap();
        let storage = FileStorage::new(yaml_format);
        storage.save(test_struct).unwrap();
        println!("{:?}", storage);
        // let storage = FileStorage::new("test.yaml");
        // let project = ProjectBuilder::new()
        //     .key("TEST")
        //     .name("Test Project")
        //     .description("A test project")
        //     .build()
        //     .unwrap();
        // let id = storage.create_project(project).unwrap();
        // let retrieved = storage.get_project(id).unwrap();
        // assert_eq!(retrieved.key(), "TEST");
        // assert_eq!(retrieved.name(), "Test Project");
        // assert_eq!(retrieved.description(), "A test project");
    }
}
