use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProjectId(Uuid);

impl From<Uuid> for ProjectId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<ProjectId> for Uuid {
    fn from(id: ProjectId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone)]
pub struct ProjectKey(String);

impl From<String> for ProjectKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}

impl From<ProjectKey> for String {
    fn from(key: ProjectKey) -> Self {
        key.0
    }
}

#[derive(Debug, Clone)]
pub struct ProjectName(String);

impl From<String> for ProjectName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<ProjectName> for String {
    fn from(name: ProjectName) -> Self {
        name.0
    }
}

#[derive(Debug, Clone)]
pub struct ProjectDescription(String);

impl From<String> for ProjectDescription {
    fn from(description: String) -> Self {
        Self(description)
    }
}

impl From<ProjectDescription> for String {
    fn from(description: ProjectDescription) -> Self {
        description.0
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub key: ProjectKey,
    pub name: ProjectName,
    pub description: ProjectDescription,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(id: Uuid, key: String, name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: ProjectId::from(id),
            key: ProjectKey::from(key),
            name: ProjectName::from(name),
            description: ProjectDescription::from(description),
            created_at: now,
            updated_at: now,
        }
    }
}
