use chrono::{TimeZone, Utc};
use uuid::Uuid;

use core::{
    domain::{
        errors::CreateProjectError,
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
    ) -> Result<NewProject, CreateProjectError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| CreateProjectError::Unknown)?;

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
        .map(|row| {
            Ok(NewProject {
                id: Uuid::from_str(&row.id).map_err(|_| CreateProjectError::Unknown)?,
                key: row.key,
                name: row.name,
                description: row.description,
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(project) => project,
            Err(e) => {
                println!("Error inserting project {}", e);
                Err(CreateProjectError::Unknown)
            }
        }
    }
}
