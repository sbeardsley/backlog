use crate::domain::{
    errors::CreateProjectError,
    models::{NewProject, ProjectDraft},
};

pub trait CreateProjectUseCase: Send + Sync {
    fn create_project(
        &self,
        project: ProjectDraft,
    ) -> impl std::future::Future<Output = Result<NewProject, CreateProjectError>> + Send;
}
