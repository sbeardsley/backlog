use crate::{
    entities::project::Project,
    interfaces::{
        repositories::projects::create_post::CreateProjectRepository,
        use_cases::projects::create_project::{
            CreateProjectError, CreateProjectUseCase, ProjectDraft,
        },
    },
};

pub struct CreateProject<T: CreateProjectRepository> {
    repository: T,
}

impl<T: CreateProjectRepository> CreateProject<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: CreateProjectRepository> CreateProjectUseCase for CreateProject<T> {
    async fn execute(&self, project: ProjectDraft) -> Result<Project, CreateProjectError> {
        self.repository.create_project(project).await
    }
}
