use crate::repositories::{
    create_project_repository::CreateProjectRepository,
    sqlite_connection_pool::SqliteConnectionPool,
};
use core::{
    app::{contracts::CreateProjectCommandHandler, services::CreateProjectService},
    domain::models::CreateProjectCommand,
};
use std::sync::Arc;
pub struct App {
    pool: SqliteConnectionPool,
    create_command_handler: CreateProjectService<CreateProjectRepository>,
}

impl App {
    pub async fn new(db_url: &str) -> Self {
        let pool = SqliteConnectionPool::new(db_url).await.unwrap();
        let repository = CreateProjectRepository::new(Arc::new(pool.clone()));

        return Self {
            pool,
            create_command_handler: CreateProjectService::new(repository),
        };
    }
    pub async fn run(&self, project: CreateProjectCommand) {
        self.create_command_handler
            .handle(project)
            .await
            .expect("Failed to create project");

        println!("Project created successfully");
        self.pool.close().await;
    }
}
