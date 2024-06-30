use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetOneProjectError, models::NewProject},
    repositories::GetOneProjectRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetOneProjectRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetOneProjectRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetOneProjectRepositoryContract for GetOneProjectRepository {
    async fn get_one_project(
        &self,
        project_id: Uuid,
    ) -> Result<Option<NewProject>, GetOneProjectError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetOneProjectError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, key, name, description, created_at, updated_at
            FROM projects
            WHERE id = $1
            "#,
            project_id,
        )
        .map(|row| -> Option<NewProject> {
            let id = Uuid::from_str(&row.id).ok();

            if Some(id) == None {
                return None;
            }

            Some(NewProject {
                id: Uuid::from_str(&row.id).expect("unable to parse id"),
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
            Ok(project) => Ok(project),
            Err(_) => Err(GetOneProjectError::Unknown),
        }
    }
}
