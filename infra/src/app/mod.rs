use crate::repositories::{
    sqlite_connection_pool::SqliteConnectionPool, CreateProjectRepository,
    GetAllProjectsRepository, GetOneProjectRepository, UpdateProjectRepository,
};
use core::{
    app::{
        contracts::{
            CreateProjectCommandHandler, GetAllProjectsQueryHandler, GetOneProjectQueryHandler,
            UpdateProjectCommandHandler,
        },
        services::{
            CreateProjectService, GetAllProjectsService, GetOneProjectService, UpdateProjectService,
        },
    },
    domain::{
        errors::{GetAllProjectsError, GetOneProjectError},
        models::{
            CreateProjectCommand, GetAllProjectsQuery, GetAllProjectsQueryResult,
            GetOneProjectQuery, GetOneProjectQueryResult, UpdateProjectCommand,
        },
    },
};
use std::sync::Arc;

pub struct App {
    pool: Arc<SqliteConnectionPool>,
    create_project_command_handler: CreateProjectService<CreateProjectRepository>,
    update_project_command_handler: UpdateProjectService<UpdateProjectRepository>,
    get_one_project_command_handler: GetOneProjectService<GetOneProjectRepository>,
    get_all_projects_command_handler: GetAllProjectsService<GetAllProjectsRepository>,
}

impl App {
    pub async fn new(db_url: &str) -> Self {
        let pool = Arc::new(SqliteConnectionPool::new(db_url).await.unwrap());
        let create_project_repository = CreateProjectRepository::new(pool.clone());
        let update_project_repository = UpdateProjectRepository::new(pool.clone());
        let get_one_project_repository = GetOneProjectRepository::new(pool.clone());
        let get_all_projects_repository = GetAllProjectsRepository::new(pool.clone());

        return Self {
            pool,
            create_project_command_handler: CreateProjectService::new(create_project_repository),
            update_project_command_handler: UpdateProjectService::new(update_project_repository),
            get_one_project_command_handler: GetOneProjectService::new(get_one_project_repository),
            get_all_projects_command_handler: GetAllProjectsService::new(
                get_all_projects_repository,
            ),
        };
    }

    pub async fn create_project(&self, project: CreateProjectCommand) {
        self.create_project_command_handler
            .handle(project)
            .await
            .expect("Failed to create project");

        println!("Project created successfully");
        self.pool.close().await;
    }

    pub async fn update_project(&self, project: UpdateProjectCommand) {
        self.update_project_command_handler
            .handle(project)
            .await
            .expect("Failed to create project");

        println!("Project created successfully");
        self.pool.close().await;
    }

    pub async fn get_project(
        &self,
        project: GetOneProjectQuery,
    ) -> Result<GetOneProjectQueryResult, GetOneProjectError> {
        let result = self.get_one_project_command_handler.handle(project).await?;

        self.pool.close().await;

        Ok(result)
    }

    pub async fn get_all_projects(
        &self,
        project: GetAllProjectsQuery,
    ) -> Result<Vec<GetAllProjectsQueryResult>, GetAllProjectsError> {
        let result = self
            .get_all_projects_command_handler
            .handle(project)
            .await?;

        self.pool.close().await;

        Ok(result)
    }
}
