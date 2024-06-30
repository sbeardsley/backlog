use crate::domain::{
    errors::GetOneProjectError,
    models::{GetOneProjectQuery, GetOneProjectQueryResult},
};

pub trait GetOneProjectQueryHandler {
    fn handle(
        &self,
        query: GetOneProjectQuery,
    ) -> impl std::future::Future<Output = Result<GetOneProjectQueryResult, GetOneProjectError>> + Send;
}
