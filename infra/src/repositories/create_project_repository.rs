use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::InsertProjectError,
        models::{NewProject, ProjectDraft},
    },
    repositories::CreateProjectRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct CreateProjectRepository {
    db: Arc<SqliteConnectionPool>,
}

impl CreateProjectRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl CreateProjectRepositoryContract for CreateProjectRepository {
    async fn insert_project(
        &self,
        project: ProjectDraft,
    ) -> Result<NewProject, InsertProjectError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| InsertProjectError::Unknown)?;

        let id = Uuid::now_v7().to_string();
        let now = Utc::now();

        let key = project.key;
        let name = project.name;
        let description = project.description;
        let created_at = now;
        let updated_at = now;

        match sqlx::query!(
            r#"
            INSERT INTO projects (id, key, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, key, name, description, created_at, updated_at;
            "#,
            id,
            key,
            name,
            description,
            created_at,
            updated_at,
        )
        .map(|row| NewProject {
            id: Uuid::from_str(&row.id.unwrap()).unwrap(),
            key: row.key,
            name: row.name,
            description: row.description.unwrap_or_default(),
            created_at: Utc.from_utc_datetime(&row.created_at.unwrap()),
            updated_at: Utc.from_utc_datetime(&row.updated_at.unwrap()),
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(project) => Ok(project),
            Err(e) => {
                println!("Error inserting project {}", e);
                Err(InsertProjectError::Unknown)
            }
        }
    }
}
