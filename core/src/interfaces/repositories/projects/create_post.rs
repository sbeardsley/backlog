use crate::{
    entities::project::Project,
    interfaces::use_cases::projects::create_project::{CreateProjectError, ProjectDraft},
};

pub trait CreateProjectRepository: Send + Sync {
    async fn create_project(&self, project: ProjectDraft) -> Result<Project, CreateProjectError>;
}
