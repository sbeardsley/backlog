use crate::models::{
    dto::{NewProject, ProjectDraft},
    errors::CreateProjectError,
};

pub trait CreateProjectUseCase: Send + Sync {
    fn execute(
        &self,
        project: ProjectDraft,
    ) -> impl std::future::Future<Output = Result<NewProject, CreateProjectError>> + Send;
}
