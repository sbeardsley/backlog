use clap::Args;
use core::domain::models::{CreateProjectCommand, GetOneProjectQuery};
use infra::app::App;

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

pub async fn run(project: GetOneProjectQuery) {
    let app = App::new("sqlite://.backlog.db").await;
    app.get_project(project).await;
}
