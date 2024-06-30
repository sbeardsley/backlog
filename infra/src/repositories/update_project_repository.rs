use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::UpdateProjectError,
        models::{NewProject, ProjectPatch},
    },
    repositories::UpdateProjectRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct UpdateProjectRepository {
    db: Arc<SqliteConnectionPool>,
}

impl UpdateProjectRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl UpdateProjectRepositoryContract for UpdateProjectRepository {
    async fn update_project(
        &self,
        project_id: Uuid,
        project: ProjectPatch,
    ) -> Result<NewProject, UpdateProjectError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| UpdateProjectError::Unknown)?;

        let id = Uuid::now_v7().to_string();
        let key = project.key;
        let name = project.name;
        let description = project.description;

        match sqlx::query!(
            r#"
            UPDATE projects
            SET name = $2, key = $3, description = $4
            WHERE id = $1
            RETURNING id, name, key, description, created_at, updated_at;
            "#,
            id,
            name,
            key,
            description,
        )
        .map(|row| {
            Ok(NewProject {
                id: Uuid::from_str(&row.id).map_err(|_| UpdateProjectError::Unknown)?,
                name: row.name,
                key: row.key,
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
                Err(UpdateProjectError::Unknown)
            }
        }
    }
}
