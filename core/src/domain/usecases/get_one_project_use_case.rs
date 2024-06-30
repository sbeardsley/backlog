use crate::domain::{errors::GetOneProjectError, models::NewProject};
use uuid::Uuid;

pub trait GetOneProjectUseCase: Send + Sync {
    fn get_one_project(
        &self,
        project_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewProject>, GetOneProjectError>> + Send;
}
