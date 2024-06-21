use core::{
    entities::project::Project,
    interfaces::{
        repositories::projects::create_post::CreateProjectRepository,
        use_cases::projects::create_project::{CreateProjectError, ProjectDraft},
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
    async fn create_project(&self, project: ProjectDraft) -> Result<Project, CreateProjectError> {
        let mut lock = match self.projects.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(CreateProjectError::Unknown),
        };

        let mut project = Project::new(project.key, project.name, project.description);
        project.id = (lock.len() as u32) + 1;

        lock.push(project.clone());
        Ok(project)
    }
}
