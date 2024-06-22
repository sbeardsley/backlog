use serde::{Deserialize, Serialize};

use crate::models::errors::CreateProjectError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectCommand {
    pub key: String,
    pub name: String,
    pub description: String,
}

pub trait CreateProjectCommandHandler {
    fn handle(
        &self,
        command: CreateProjectCommand,
    ) -> impl std::future::Future<Output = Result<(), CreateProjectError>> + Send;
}
