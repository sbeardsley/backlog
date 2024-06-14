use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "backlog", bin_name = "backlog")]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
/// A command-line interface to interact with the backlog board.
pub(crate) enum Commands {
    /// Configure the backlog application.
    Config(ConfigArgs),
    /// Create a new record type.
    Create(CreateArgs),
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

// Region: Config

#[derive(Debug, Args)]
pub(crate) struct ConfigArgs {
    #[command(subcommand)]
    pub(crate) command: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum StorageCommands {
    /// Use a file storage backend.
    File(FileStorageArgs),
    /// Use a database storage backend.
    Db(DbStorageArgs),
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub(crate) enum FileFormat {
    Yaml,
    Json,
    Toml,
}

#[derive(Debug, Args)]
pub(crate) struct FileStorageArgs {
    /// The path to the file.
    pub(crate) file: String,
    /// The format of the file.
    #[arg(short, long)]
    #[clap(value_enum)]
    pub(crate) format: FileFormat,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub(crate) enum DbType {
    Surreal,
    Postgres,
    Mysql,
    Sqlite,
    Rocks,
}

#[derive(Debug, Args)]
pub(crate) struct DbStorageArgs {
    /// The db url or connection string.
    pub(crate) db_url: String,
    /// The type of database to use.
    #[arg(short, long)]
    #[clap(value_enum)]
    pub(crate) db_type: DbType,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ConfigCommands {
    /// The storage type to use.
    Storage {
        #[command(subcommand)]
        storage: Option<StorageCommands>,
    },
}

// EndRegion: Config

// Region: Create
#[derive(Debug, Args)]
pub(crate) struct CreateArgs {
    #[command(subcommand)]
    pub(crate) command: Option<CreateCommands>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CreateCommands {
    /// Create a new project.
    Project {
        /// The name of the project.
        name: String,
        /// The key to use for tickets (3 char max).
        key: String,
        /// The description of the project.
        #[arg(short, long)]
        description: Option<String>,
    },
}
// EndRegion: Create
