use clap::Args;
use core::{
    app::{contracts::CreateProjectCommandHandler, services::CreateProjectService},
    domain::models::CreateProjectCommand,
};
use infra::repositories::{
    create_project_repository::CreateProjectRepository,
    sqlite_connection_pool::SqliteConnectionPool,
};
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct ProjectAddArgs {
    /// The key of the project
    key: String,
    /// The name of the project
    name: String,
    /// The description of the project
    description: String,
}

impl From<ProjectAddArgs> for CreateProjectCommand {
    fn from(args: ProjectAddArgs) -> Self {
        CreateProjectCommand {
            key: args.key,
            name: args.name,
            description: args.description,
        }
    }
}

pub async fn run(project: CreateProjectCommand) {
    let db_url = "sqlite://.backlog.db";
    let pool = SqliteConnectionPool::new(db_url).await.unwrap();
    let repository = CreateProjectRepository::new(Arc::new(pool.clone()));
    let create_project_service = CreateProjectService::new(repository);
    create_project_service
        .handle(project)
        .await
        .expect("Failed to create project");
    println!("Project created successfully");
    pool.close().await;
}
