use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::Project;

#[derive(Debug)]
pub struct ProjectDraft {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct CreateProjectCommand {
    pub key: String,
    pub name: String,
    pub description: String,
}

impl From<CreateProjectCommand> for ProjectDraft {
    fn from(command: CreateProjectCommand) -> Self {
        ProjectDraft {
            key: command.key,
            name: command.name,
            description: command.description,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NewProject {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Project> for NewProject {
    fn from(project: Project) -> Self {
        NewProject {
            id: project.id.into(),
            key: project.key.into(),
            name: project.name.into(),
            description: project.description.into(),
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
