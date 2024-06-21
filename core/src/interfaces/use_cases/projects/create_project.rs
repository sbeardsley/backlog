use crate::entities::project::Project;

pub struct ProjectDraft {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub enum CreateProjectError {
    ProjectAlreadyExists,
    Unknown,
}

pub trait CreateProjectUseCase: Send + Sync {
    async fn execute(&self, project: ProjectDraft) -> Result<Project, CreateProjectError>;
}
