use crate::{
    domain::{errors::GetOneProjectError, models::NewProject, usecases::GetOneProjectUseCase},
    repositories::GetOneProjectRepositoryContract,
};
use uuid::Uuid;

pub struct GetOneProjectService<R: GetOneProjectRepositoryContract> {
    repository: R,
}

impl<R: GetOneProjectRepositoryContract> GetOneProjectService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetOneProjectRepositoryContract> GetOneProjectUseCase for GetOneProjectService<R> {
    async fn get_one_project(
        &self,
        project_id: Uuid,
    ) -> Result<Option<NewProject>, GetOneProjectError> {
        self.repository.get_one_project(project_id).await
    }
}
