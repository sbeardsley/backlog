mod cli;
use clap::{Args, Parser, Subcommand};
use cli::project::{
    create_project::{self, ProjectAddArgs},
    get_all_projects, update_project,
};
use core::domain::models::CreateProjectCommand;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Do something with a project
    Project(ProjectArgs),

    /// Do something with a board
    Board,

    /// Do something with a task
    Issue,
}

#[derive(Debug, Args)]
struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectSubcommands,
}

#[derive(Debug, Subcommand)]
enum ProjectSubcommands {
    /// Create a new project
    #[command(arg_required_else_help = true)]
    Add(ProjectAddArgs),

    /// List all projects
    List,

    /// Update a project
    Update,

    /// Delete a project
    Delete,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Project(project_args)) => match project_args.command {
            ProjectSubcommands::Add(args) => {
                create_project::run(CreateProjectCommand::from(args)).await
            }
            ProjectSubcommands::List => get_all_projects::run().await,
            ProjectSubcommands::Update => println!("Updating a project"),
            ProjectSubcommands::Delete => println!("Deleting a project"),
        },
        Some(Commands::Board) => println!("Board command"),
        Some(Commands::Issue) => println!("Issue command"),
        None => println!("No command provided"),
    }
}
