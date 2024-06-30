use uuid::Uuid;

use crate::domain::{errors::GetOneProjectError, models::NewProject};

pub trait GetOneProjectRepositoryContract: Clone + Send + Sync + 'static {
    fn get_one_project(
        &self,
        project_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewProject>, GetOneProjectError>> + Send;
}
