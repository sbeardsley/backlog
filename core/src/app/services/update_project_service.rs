use crate::{
    app::contracts::UpdateProjectCommandHandler,
    domain::{
        errors::UpdateProjectError,
        models::{ProjectPatch, UpdateProjectCommand},
        usecases::UpdateProjectUseCase,
    },
    repositories::UpdateProjectRepositoryContract,
};

pub struct UpdateProjectService<T: UpdateProjectRepositoryContract> {
    update_project: crate::domain::services::UpdateProjectService<T>,
}

impl<T: UpdateProjectRepositoryContract> UpdateProjectService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            update_project: crate::domain::services::UpdateProjectService::new(repository),
        }
    }
}

impl<T: UpdateProjectRepositoryContract> UpdateProjectCommandHandler for UpdateProjectService<T> {
    async fn handle(&self, command: UpdateProjectCommand) -> Result<(), UpdateProjectError> {
        match self
            .update_project
            .update_project(command.id, ProjectPatch::from(command))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
