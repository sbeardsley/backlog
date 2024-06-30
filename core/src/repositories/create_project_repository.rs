use crate::domain::{
    errors::CreateProjectError,
    models::{NewProject, ProjectDraft},
};

pub trait CreateProjectRepositoryContract: Clone + Send + Sync + 'static {
    fn insert_project(
        &self,
        project: ProjectDraft,
    ) -> impl std::future::Future<Output = Result<NewProject, CreateProjectError>> + Send;
}
