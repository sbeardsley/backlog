pub mod error;
use std::fmt::Display;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use error::ValidationError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ProjectId = u32;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ProjectKey(String);

impl ProjectKey {
    pub fn new(key: impl Into<String>) -> Result<Self, ValidationError> {
        let key = key.into();
        if key.is_empty() {
            return Err(ValidationError::new("Key is required"));
        }
        if key.len() > 3 {
            return Err(ValidationError::new(
                "Key cannot be longer than 3 characters",
            ));
        }
        Ok(Self(key))
    }
}

impl Display for ProjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: impl Into<String>) -> Result<Self, ValidationError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ValidationError::new("Name is required"));
        }
        if name.len() > 256 {
            return Err(ValidationError::new(
                "Name cannot be longer than 256 characters",
            ));
        }
        Ok(Self(name))
    }
}

impl Display for ProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ProjectDescription(String);

impl ProjectDescription {
    pub fn new(desc: impl Into<String>) -> Result<Self, ValidationError> {
        let desc = desc.into();
        if desc.is_empty() {
            return Err(ValidationError::new(
                "Empty String is not a valid description",
            ));
        }
        if desc.len() > 255 {
            return Err(ValidationError::new(
                "Description cannot be longer than 256 characters",
            ));
        }
        Ok(Self(desc))
    }
}

impl Display for ProjectDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

pub struct ProjectDraft {
    pub name: ProjectName,
    pub key: ProjectKey,
    pub description: ProjectDescription,
}

impl ProjectDraft {
    pub fn new(name: ProjectName, key: ProjectKey, description: ProjectDescription) -> Self {
        ProjectDraft {
            name,
            key,
            description,
        }
    }
}

pub struct ProjectPatch {
    pub name: Option<ProjectName>,
    pub key: Option<ProjectKey>,
    pub description: Option<ProjectDescription>,
}

impl ProjectPatch {
    pub fn new(name: ProjectName, key: ProjectKey, description: ProjectDescription) -> Self {
        ProjectPatch {
            name: Some(name),
            key: Some(key),
            description: Some(description),
        }
    }
}

pub struct DeletedProject {
    pub project: Project,
    #[cfg(feature = "chrono")]
    pub deleted_at: DateTime<Utc>,
}

impl DeletedProject {
    pub fn new(project: Project) -> Self {
        DeletedProject {
            project,
            #[cfg(feature = "chrono")]
            deleted_at: Utc::now(),
        }
    }
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    key: ProjectKey,
    name: ProjectName,
    description: ProjectDescription,
    #[cfg(feature = "chrono")]
    created_at: DateTime<Utc>,
    #[cfg(feature = "chrono")]
    updated_at: DateTime<Utc>,
}

impl Project {
    pub fn id(&self) -> ProjectId {
        self.id
    }

    pub fn key(&self) -> &ProjectKey {
        &self.key
    }

    pub fn name(&self) -> &ProjectName {
        &self.name
    }

    pub fn description(&self) -> &ProjectDescription {
        &self.description
    }

    #[cfg(feature = "chrono")]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    #[cfg(feature = "chrono")]
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// region:  -- States
#[derive(Default, Clone)]
pub struct NoName;
#[derive(Default, Clone)]
pub struct NoKey;
#[derive(Default, Clone)]
pub struct NoDescription;
#[derive(Default, Clone)]
pub struct Name(String);
#[derive(Default, Clone)]
pub struct Key(String);
#[derive(Default, Clone)]
pub struct Description(String);
// endregion: -- States

#[derive(Default, Clone)]
pub struct ProjectBuilder<K, N, D> {
    id: Option<ProjectId>,
    key: K,
    name: N,
    description: D,
    #[cfg(feature = "chrono")]
    created_at: Option<DateTime<Utc>>,
    #[cfg(feature = "chrono")]
    updated_at: Option<DateTime<Utc>>,
}

impl ProjectBuilder<NoKey, NoName, NoDescription> {
    pub fn new() -> Self {
        ProjectBuilder::default()
    }
}

impl ProjectBuilder<Key, Name, Description> {
    pub fn build(self) -> Result<Project, ValidationError> {
        #[cfg(feature = "chrono")]
        let created_at = self.created_at.unwrap_or_else(Utc::now);
        #[cfg(feature = "chrono")]
        let updated_at = self.updated_at.unwrap_or_else(Utc::now);

        let id = self.id.unwrap_or(0);
        let key = ProjectKey::new(self.key.0)?;
        let name = ProjectName::new(self.name.0)?;
        let description = ProjectDescription::new(self.description.0)?;

        Ok(Project {
            id,
            key,
            name,
            description,
            #[cfg(feature = "chrono")]
            created_at,
            #[cfg(feature = "chrono")]
            updated_at,
        })
    }

    pub fn from_project(project: Project) -> Self {
        ProjectBuilder {
            id: Some(project.id),
            key: Key(project.key.0),
            name: Name(project.name.0),
            description: Description(project.description.0),
            #[cfg(feature = "chrono")]
            created_at: Some(project.created_at),
            #[cfg(feature = "chrono")]
            updated_at: Some(project.updated_at),
        }
    }
}

impl<K, N, D> ProjectBuilder<K, N, D> {
    pub fn id(mut self, id: ProjectId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn key(self, key: impl Into<String>) -> ProjectBuilder<Key, N, D> {
        ProjectBuilder {
            id: self.id,
            key: Key(key.into()),
            name: self.name,
            description: self.description,
            #[cfg(feature = "chrono")]
            created_at: self.created_at,
            #[cfg(feature = "chrono")]
            updated_at: self.updated_at,
        }
    }

    pub fn name(self, name: impl Into<String>) -> ProjectBuilder<K, Name, D> {
        ProjectBuilder {
            id: self.id,
            key: self.key,
            name: Name(name.into()),
            description: self.description,
            #[cfg(feature = "chrono")]
            created_at: self.created_at,
            #[cfg(feature = "chrono")]
            updated_at: self.updated_at,
        }
    }

    pub fn description(self, desc: impl Into<String>) -> ProjectBuilder<K, N, Description> {
        ProjectBuilder {
            id: self.id,
            key: self.key,
            name: self.name,
            description: Description(desc.into()),
            #[cfg(feature = "chrono")]
            created_at: self.created_at,
            #[cfg(feature = "chrono")]
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_project_key_valid() {
        let key = ProjectKey::new("KEY");
        assert!(key.is_ok(), "{:?}", key);
    }

    #[test]
    fn test_project_key_must_not_be_empty() {
        let key = ProjectKey::new("");
        assert!(key.is_err(), "{:?}", key);
    }

    #[test]
    fn test_project_key_must_be_less_than_3_characters() {
        let key = ProjectKey::new("KEYKEY");
        assert!(key.is_err(), "{:?}", key);
    }

    #[test]
    fn test_project_name_valid() {
        let name = ProjectName::new("Name");
        assert!(name.is_ok(), "{:?}", name.err());
    }

    #[test]
    fn test_project_name_must_not_be_empty() {
        let name = ProjectName::new("");
        assert!(name.is_err(), "{:?}", name.err());
    }

    #[test]
    fn test_project_name_must_be_less_than_256_characters() {
        let name = ProjectName::new((0..257).map(|_| "X").collect::<String>());
        assert!(name.is_err(), "{:?}", name.err());
    }

    #[test]
    fn test_project_description_valid() {
        let name = ProjectDescription::new("Name");
        assert!(name.is_ok(), "{:?}", name.err());
    }

    #[test]
    fn test_project_description_must_not_be_empty() {
        let name = ProjectDescription::new("");
        assert!(name.is_err(), "{:?}", name.err());
    }

    #[test]
    fn test_project_description_must_be_less_than_256_characters() {
        let name = ProjectDescription::new((0..256).map(|_| "X").collect::<String>());
        assert!(name.is_err(), "{:?}", name.err());
    }

    #[test]
    fn test_project_builder_valid() {
        let project = ProjectBuilder::new()
            .id(1)
            .key("KEY")
            .name("Name")
            .description("Description")
            .build();
        assert!(project.is_ok(), "{:?}", project.err());
    }

    #[test]
    fn test_project_builder_invalid() {
        let project = ProjectBuilder::new()
            .id(1)
            .key("")
            .name("")
            .description("")
            .build();
        assert!(project.is_err(), "{:?}", project.err());
    }
}
