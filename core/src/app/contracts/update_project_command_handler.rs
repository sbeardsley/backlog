use crate::domain::{errors::UpdateProjectError, models::UpdateProjectCommand};

pub trait UpdateProjectCommandHandler {
    fn handle(
        &self,
        command: UpdateProjectCommand,
    ) -> impl std::future::Future<Output = Result<(), UpdateProjectError>> + Send;
}
