use crate::domain::{
    errors::GetAllProjectsError,
    models::{GetAllProjectsQuery, GetAllProjectsQueryResult},
};

pub trait GetAllProjectsQueryHandler {
    fn handle(
        &self,
        query: GetAllProjectsQuery,
    ) -> impl std::future::Future<Output = Result<Vec<GetAllProjectsQueryResult>, GetAllProjectsError>>
           + Send;
}
