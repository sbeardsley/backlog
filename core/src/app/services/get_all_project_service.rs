use crate::{
    app::contracts::GetAllProjectsQueryHandler,
    domain::{
        errors::GetAllProjectsError,
        models::{GetAllProjectsQuery, GetAllProjectsQueryResult},
        usecases::GetAllProjectsUseCase,
    },
    repositories::GetAllProjectsRepositoryContract,
};

pub struct GetAllProjectsService<T: GetAllProjectsRepositoryContract> {
    get_all_projects: crate::domain::services::GetAllProjectsService<T>,
}

impl<T: GetAllProjectsRepositoryContract> GetAllProjectsService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_all_projects: crate::domain::services::GetAllProjectsService::new(repository),
        }
    }
}

impl<T: GetAllProjectsRepositoryContract> GetAllProjectsQueryHandler for GetAllProjectsService<T> {
    async fn handle(
        &self,
        _: GetAllProjectsQuery,
    ) -> Result<Vec<GetAllProjectsQueryResult>, GetAllProjectsError> {
        match self.get_all_projects.get_all_projects().await {
            Ok(project) => Ok(project
                .into_iter()
                .map(|b| GetAllProjectsQueryResult::from(b))
                .collect()),
            Err(e) => Err(e),
        }
    }
}
