use crate::models::{
    dto::{NewProject, ProjectDraft},
    errors::CreateProjectError,
};

pub trait CreateProjectRepository: Send + Sync {
    fn create_project(
        &self,
        project: ProjectDraft,
    ) -> impl std::future::Future<Output = Result<NewProject, CreateProjectError>> + Send;
}
