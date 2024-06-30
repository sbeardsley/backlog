use crate::{
    domain::{
        errors::UpdateProjectError,
        models::{NewProject, ProjectPatch},
        usecases::UpdateProjectUseCase,
    },
    repositories::UpdateProjectRepositoryContract,
};
use uuid::Uuid;

pub struct UpdateProjectService<R: UpdateProjectRepositoryContract> {
    repository: R,
}

impl<R: UpdateProjectRepositoryContract> UpdateProjectService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UpdateProjectRepositoryContract> UpdateProjectUseCase for UpdateProjectService<R> {
    async fn update_project(
        &self,
        project_id: Uuid,
        project: ProjectPatch,
    ) -> Result<NewProject, UpdateProjectError> {
        self.repository.update_project(project_id, project).await
    }
}
