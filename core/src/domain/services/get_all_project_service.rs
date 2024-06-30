use crate::{
    domain::{errors::GetAllProjectsError, models::NewProject, usecases::GetAllProjectsUseCase},
    repositories::GetAllProjectsRepositoryContract,
};

pub struct GetAllProjectsService<R: GetAllProjectsRepositoryContract> {
    repository: R,
}

impl<R: GetAllProjectsRepositoryContract> GetAllProjectsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetAllProjectsRepositoryContract> GetAllProjectsUseCase for GetAllProjectsService<R> {
    async fn get_all_projects(&self) -> Result<Vec<NewProject>, GetAllProjectsError> {
        self.repository.get_all_projects().await
    }
}
