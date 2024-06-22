use core::{
    interfaces::ports::{CreateProjectCommand, CreateProjectCommandHandler},
    services::CreateProjectService,
};

use clap::Args;
use infra::repositories::project_repository::InMemoryRepository;

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
    let repository = InMemoryRepository::new();
    let service = CreateProjectService::new(repository);

    match service.handle(project).await {
        Ok(_) => println!("Project created successfully"),
        Err(e) => println!("Error creating project: {:?}", e),
    }
}
