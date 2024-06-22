use core::{
    entities::Project,
    interfaces::ports::CreateProjectRepository,
    models::{
        dto::{NewProject, ProjectDraft},
        errors::CreateProjectError,
    },
};
use std::sync::Mutex;

pub struct InMemoryRepository {
    projects: Mutex<Vec<Project>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let projects: Mutex<Vec<Project>> = Mutex::new(vec![]);
        Self { projects }
    }
}

impl CreateProjectRepository for InMemoryRepository {
    async fn create_project(
        &self,
        project: ProjectDraft,
    ) -> Result<NewProject, CreateProjectError> {
        let mut lock = match self.projects.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(CreateProjectError::Unknown),
        };

        let id = (lock.len() as u32) + 1;
        let project = Project::new(id, project.key, project.name, project.description);

        lock.push(project.clone());
        Ok(project.into())
    }
}
