use crate::{
    interfaces::{ports::CreateProjectRepository, use_cases::CreateProjectUseCase},
    models::{
        dto::{NewProject, ProjectDraft},
        errors::CreateProjectError,
    },
};

pub struct CreateProject<R: CreateProjectRepository> {
    repository: R,
}

impl<R: CreateProjectRepository> CreateProject<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: CreateProjectRepository> CreateProjectUseCase for CreateProject<R> {
    async fn execute(&self, project: ProjectDraft) -> Result<NewProject, CreateProjectError> {
        self.repository.create_project(project).await
    }
}
