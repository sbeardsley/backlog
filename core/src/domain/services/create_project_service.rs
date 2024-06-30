use crate::{
    domain::{
        errors::CreateProjectError,
        models::{NewProject, ProjectDraft},
        usecases::CreateProjectUseCase,
    },
    repositories::CreateProjectRepositoryContract,
};

pub struct CreateProjectService<R: CreateProjectRepositoryContract> {
    repository: R,
}

impl<R: CreateProjectRepositoryContract> CreateProjectService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: CreateProjectRepositoryContract> CreateProjectUseCase for CreateProjectService<R> {
    async fn create_project(
        &self,
        project: ProjectDraft,
    ) -> Result<NewProject, CreateProjectError> {
        self.repository.insert_project(project).await
    }
}
