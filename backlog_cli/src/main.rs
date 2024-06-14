mod backlog_app;
mod cli;

use backlog_app::BacklogApp;
use clap::Parser;
use cli::{Cli, Commands, ConfigCommands, CreateCommands, StorageCommands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Config(config)) => {
            let config_cmd = config
                .command
                .unwrap_or(ConfigCommands::Storage { storage: None });

            match config_cmd {
                ConfigCommands::Storage { storage } => match storage {
                    Some(StorageCommands::File(file)) => {
                        BacklogApp::config_file_storage(&file.file, file.format).unwrap();
                        println!("Config saved.")
                    }
                    Some(StorageCommands::Db(db)) => {
                        BacklogApp::config_db_storage(&db.db_url, db.db_type).unwrap();
                        println!("Config saved.")
                    }
                    None => {
                        println!("No storage type provided");
                    }
                },
            }
        }
        Some(Commands::Create(create)) => {
            let create_cmd = create.command.unwrap_or(CreateCommands::Project {
                name: "".to_string(),
                key: "".to_string(),
                description: None,
            });

            match create_cmd {
                CreateCommands::Project {
                    name,
                    key,
                    description,
                } => {
                    println!("Created project with name: {} and key: {}", name, key);
                    if let Some(desc) = description {
                        println!("Description: {}", desc);
                    }
                }
            }
        }
        None => {
            println!("No command provided");
        }
    }
}
