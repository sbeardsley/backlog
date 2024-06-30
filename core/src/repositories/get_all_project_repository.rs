use crate::domain::{errors::GetAllProjectsError, models::NewProject};

pub trait GetAllProjectsRepositoryContract: Clone + Send + Sync + 'static {
    fn get_all_projects(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<NewProject>, GetAllProjectsError>> + Send;
}
