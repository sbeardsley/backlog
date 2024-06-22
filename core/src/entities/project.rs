use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectId(u32);

impl From<u32> for ProjectId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<ProjectId> for u32 {
    fn from(id: ProjectId) -> Self {
        id.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub key: ProjectKey,
    pub name: ProjectName,
    pub description: ProjectDescription,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(id: u32, key: String, name: String, description: String) -> Self {
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
