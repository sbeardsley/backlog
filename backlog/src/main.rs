use core::{
    interfaces::use_cases::projects::create_project::{CreateProjectUseCase, ProjectDraft},
    use_cases::projects::create_project::CreateProject,
};

use infra::repositories::project_repository::InMemoryRepository;

#[tokio::main]
async fn main() {
    let repo = InMemoryRepository::new();
    let projects = CreateProject::new(repo);
    let project = ProjectDraft {
        key: "KEY".to_string(),
        name: "Name".to_string(),
        description: "Description".to_string(),
    };
    match projects.execute(project).await {
        Ok(project) => println!("Project created: {:?}", project),
        Err(err) => println!("Error creating project: {:?}", err),
    };
}
