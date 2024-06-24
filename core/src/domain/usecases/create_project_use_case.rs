use crate::domain::{
    errors::InsertProjectError,
    models::{NewProject, ProjectDraft},
};

pub trait CreateProjectUseCase: Send + Sync {
    fn create_project(
        &self,
        project: ProjectDraft,
    ) -> impl std::future::Future<Output = Result<NewProject, InsertProjectError>> + Send;
}
