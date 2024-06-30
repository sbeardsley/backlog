use crate::domain::{errors::CreateProjectError, models::CreateProjectCommand};
use uuid::Uuid;

pub trait CreateProjectCommandHandler {
    fn handle(
        &self,
        command: CreateProjectCommand,
    ) -> impl std::future::Future<Output = Result<Uuid, CreateProjectError>> + Send;
}
