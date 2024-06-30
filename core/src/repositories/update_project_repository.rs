use crate::domain::{
    errors::UpdateProjectError,
    models::{NewProject, ProjectPatch},
};
use uuid::Uuid;

pub trait UpdateProjectRepositoryContract: Clone + Send + Sync + 'static {
    fn update_project(
        &self,
        project_id: Uuid,
        project: ProjectPatch,
    ) -> impl std::future::Future<Output = Result<NewProject, UpdateProjectError>> + Send;
}
