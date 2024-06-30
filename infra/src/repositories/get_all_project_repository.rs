use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetAllProjectsError, models::NewProject},
    repositories::GetAllProjectsRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetAllProjectsRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetAllProjectsRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetAllProjectsRepositoryContract for GetAllProjectsRepository {
    async fn get_all_projects(&self) -> Result<Vec<NewProject>, GetAllProjectsError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetAllProjectsError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, key, name, description, created_at, updated_at
            FROM projects
            "#,
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
        .fetch_all(&mut *connection)
        .await
        {
            Ok(projects) => Ok(projects.into_iter().flatten().collect()),
            Err(_) => Err(GetAllProjectsError::Unknown),
        }
    }
}
