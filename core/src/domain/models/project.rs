use crate::domain::entities::Project;
use chrono::{DateTime, Utc};
use uuid::Uuid;

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

#[derive(Debug)]
pub struct ProjectDraft {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct ProjectPatch {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct DeleteProject {
    pub id: Uuid,
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

#[derive(Debug)]
pub struct UpdateProjectCommand {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
}

impl From<UpdateProjectCommand> for ProjectPatch {
    fn from(command: UpdateProjectCommand) -> Self {
        ProjectPatch {
            key: command.key,
            name: command.name,
            description: command.description,
        }
    }
}

#[derive(Debug)]
pub struct GetAllProjectsQuery {}

#[derive(Debug)]
pub struct GetAllProjectsQueryResult {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewProject> for GetAllProjectsQueryResult {
    fn from(entity: NewProject) -> Self {
        GetAllProjectsQueryResult {
            id: entity.id,
            name: entity.name,
            key: entity.key,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetOneProjectQuery {
    pub project_id: Uuid,
}

#[derive(Debug)]
pub struct GetOneProjectQueryResult {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewProject> for GetOneProjectQueryResult {
    fn from(entity: NewProject) -> Self {
        GetOneProjectQueryResult {
            id: entity.id,
            name: entity.name,
            key: entity.key,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
