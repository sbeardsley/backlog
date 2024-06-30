use crate::{
    app::contracts::CreateProjectCommandHandler,
    domain::{
        errors::CreateProjectError,
        models::{CreateProjectCommand, ProjectDraft},
        usecases::CreateProjectUseCase,
    },
    repositories::CreateProjectRepositoryContract,
};
use uuid::Uuid;

pub struct CreateProjectService<T: CreateProjectRepositoryContract> {
    create_project: crate::domain::services::CreateProjectService<T>,
}

impl<T: CreateProjectRepositoryContract> CreateProjectService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            create_project: crate::domain::services::CreateProjectService::new(repository),
        }
    }
}

impl<T: CreateProjectRepositoryContract> CreateProjectCommandHandler for CreateProjectService<T> {
    async fn handle(&self, command: CreateProjectCommand) -> Result<Uuid, CreateProjectError> {
        match self
            .create_project
            .create_project(ProjectDraft::from(command))
            .await
        {
            Ok(p) => Ok(p.id),
            Err(e) => Err(e),
        }
    }
}
