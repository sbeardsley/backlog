use crate::{
    app::contracts::GetOneProjectQueryHandler,
    domain::{
        errors::GetOneProjectError,
        models::{GetOneProjectQuery, GetOneProjectQueryResult},
        usecases::GetOneProjectUseCase,
    },
    repositories::GetOneProjectRepositoryContract,
};

pub struct GetOneProjectService<T: GetOneProjectRepositoryContract> {
    get_one_project: crate::domain::services::GetOneProjectService<T>,
}

impl<T: GetOneProjectRepositoryContract> GetOneProjectService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_one_project: crate::domain::services::GetOneProjectService::new(repository),
        }
    }
}

impl<T: GetOneProjectRepositoryContract> GetOneProjectQueryHandler for GetOneProjectService<T> {
    async fn handle(
        &self,
        query: GetOneProjectQuery,
    ) -> Result<GetOneProjectQueryResult, GetOneProjectError> {
        match self.get_one_project.get_one_project(query.project_id).await {
            Ok(project) => match project {
                Some(project) => Ok(GetOneProjectQueryResult::from(project)),
                None => Err(GetOneProjectError::Unknown),
            },
            Err(e) => Err(e),
        }
    }
}
