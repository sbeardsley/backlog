use backlog::builder::BacklogBuilder;
use backlog::config::{BacklogConfig, FileType};
use backlog::storage::traits::Storage;
use backlog::{prelude::*, Backlog};
use clap::{Args, Command, Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(name = "backlog")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
/// A command-line interface to interact with the backlog board.
enum Commands {
    /// Configure the backlog application.
    Config(ConfigArgs),
    // /// Initialize a new backlog board.
    // Init {
    //     /// The name of the backlog board.
    //     name: String,
    //     /// The key to use for tickets.
    //     key: String,
    // },
    // /// Create a ticket on the board.
    // Create {
    //     /// The title of the ticket.
    //     title: TicketTitle,
    //     /// The description of the ticket.
    //     #[arg(short, long)]
    //     description: Option<TicketDescription>,
    // },
    // /// List all tickets on the board.
    // List {},
    // /// Update a ticket on the board.
    // Update {
    //     /// The ID of the ticket to update.
    //     id: TicketId,
    //     /// The new title of the ticket.
    //     #[arg(short, long)]
    //     title: Option<TicketTitle>,
    //     /// The new description of the ticket.
    //     #[arg(short, long)]
    //     description: Option<TicketDescription>,
    //     /// The new status of the ticket.
    //     status: Option<Status>,
    // },
    // /// Delete a ticket from the board.
    // Delete {
    //     /// The ID of the ticket to delete.
    //     id: TicketId,
    // },
}

#[derive(Debug, Args)]
struct ConfigArgs {
    #[command(subcommand)]
    command: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
enum StorageCommands {
    /// Use a file storage backend.
    File(FileStorageArgs),
    /// Use a database storage backend.
    Db(DbStorageArgs),
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum FileFormat {
    Yaml,
    Json,
    Toml,
}

#[derive(Debug, Args)]
struct FileStorageArgs {
    /// The path to the file.
    file: String,
    /// The format of the file.
    #[arg(short, long)]
    #[clap(value_enum)]
    format: FileFormat,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum DbType {
    Surreal,
    Postgres,
    Mysql,
    Sqlite,
    Rocks,
}

#[derive(Debug, Args)]
struct DbStorageArgs {
    /// The db url or connection string.
    db_url: String,
    /// The type of database to use.
    #[arg(short, long)]
    #[clap(value_enum)]
    db_type: DbType,
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    /// The storage type to use.
    Storage {
        #[command(subcommand)]
        storage: Option<StorageCommands>,
    },
}

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
                        let config = BacklogConfig::default()
                            .use_file_storage(
                                match file.format {
                                    FileFormat::Yaml => FileType::Yaml,
                                    FileFormat::Json => FileType::Json,
                                    FileFormat::Toml => FileType::Toml,
                                },
                                &file.file,
                            )
                            .unwrap();
                        config.save().unwrap();
                        println!("Config saved to {}", file.file);
                    }
                    Some(StorageCommands::Db(db)) => {
                        // let config = BacklogConfig::default();
                    }
                    None => {
                        println!("No storage type provided");
                    }
                },
            }
        }
        None => {
            println!("No command provided");
        }
    }
}
