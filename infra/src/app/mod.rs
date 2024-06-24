use core::app::services::CreateProjectService;
use core::repositories::CreateProjectRepositoryContract;
use std::sync::Arc;

use crate::repositories::{
    create_project_repository::CreateProjectRepository,
    sqlite_connection_pool::SqliteConnectionPool,
};

pub struct App {
    pool: Arc<SqliteConnectionPool>,
    // create_project_service: CreateProjectService,
}

impl App {
    pub async fn run() {
        let db_url = "sqlite://.backlog.db";
        let pool = Arc::new(SqliteConnectionPool::new(db_url).await.unwrap());
        let repository = CreateProjectRepository::new(pool);
        // let create_project_service = CreateProjectService::new(repository).await;
    }
}
