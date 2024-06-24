use uuid::Uuid;

use crate::domain::{errors::InsertProjectError, models::CreateProjectCommand};

pub trait CreateProjectCommandHandler {
    fn handle(
        &self,
        command: CreateProjectCommand,
    ) -> impl std::future::Future<Output = Result<Uuid, InsertProjectError>> + Send;
}
