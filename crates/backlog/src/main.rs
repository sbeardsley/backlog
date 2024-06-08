use backlog::app::BacklogApp;
use clap::{Parser, Subcommand};
use cli_table::Table;
use cli_table::{print_stdout, WithTitle};
use ticketing::model::{Status, TicketDescription, TicketId, TicketTitle};

#[derive(Table)]
pub struct TicketRow {
    #[table(title = "ID")]
    pub id: u128,
    #[table(title = "Key")]
    pub key: String,
    #[table(title = "Title")]
    pub title: String,
    #[table(title = "Status")]
    pub status: Status,
    #[table(title = "Description")]
    pub description: String,
}

impl From<&ticketing::model::Ticket> for TicketRow {
    fn from(ticket: &ticketing::model::Ticket) -> Self {
        TicketRow {
            id: ticket.id().clone(),
            key: ticket.key().clone(),
            title: ticket.title().0.clone(),
            status: ticket.status().clone(),
            description: ticket.description().0.clone(),
        }
    }
}

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
    /// Initialize a new backlog board.
    Init {
        /// The name of the backlog board.
        name: String,
        /// The key to use for tickets.
        key: String,
    },
    /// Create a ticket on the board.
    Create {
        /// The title of the ticket.
        title: TicketTitle,
        /// The description of the ticket.
        #[arg(short, long)]
        description: Option<TicketDescription>,
    },
    /// List all tickets on the board.
    List {},
    /// Update a ticket on the board.
    Update {
        /// The ID of the ticket to update.
        id: TicketId,
        /// The new title of the ticket.
        #[arg(short, long)]
        title: Option<TicketTitle>,
        /// The new description of the ticket.
        #[arg(short, long)]
        description: Option<TicketDescription>,
        /// The new status of the ticket.
        status: Option<Status>,
    },
    /// Delete a ticket from the board.
    Delete {
        /// The ID of the ticket to delete.
        id: TicketId,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Init { name, key }) => {
            BacklogApp::init(name, key);
        }
        Some(Commands::Create { title, description }) => {
            let mut app = match BacklogApp::new() {
                Ok(app) => app,
                Err(init_required_error) => {
                    println!("{}", init_required_error);
                    return;
                }
            };
            let ticket_id = app.create_ticket(title, description);
            println!("Created ticket with id: {}", ticket_id);
        }
        Some(Commands::List {}) => {
            let app = match BacklogApp::new() {
                Ok(app) => app,
                Err(init_required_error) => {
                    println!("{}", init_required_error);
                    return;
                }
            };

            let mut tickets = app.list_tickets();
            tickets.sort_by_key(|ticket| ticket.id());

            let sorted = tickets
                .iter()
                .map(|ticket| TicketRow::from(*ticket))
                .collect::<Vec<TicketRow>>();
            print_stdout(sorted.with_title()).unwrap();
        }
        Some(Commands::Update {
            id,
            title,
            description,
            status,
        }) => {
            let mut app = match BacklogApp::new() {
                Ok(app) => app,
                Err(init_required_error) => {
                    println!("{}", init_required_error);
                    return;
                }
            };

            let patch = ticketing::model::TicketPatch {
                title,
                description,
                status,
            };
            match app.update_ticket(&id, patch) {
                Some(_) => {
                    println!("Updated ticket with id: {}", id);
                }
                None => {
                    println!("Ticket with id {} not found", id);
                }
            }
        }
        Some(Commands::Delete { id }) => {
            let mut app = match BacklogApp::new() {
                Ok(app) => app,
                Err(init_required_error) => {
                    println!("{}", init_required_error);
                    return;
                }
            };

            match app.delete_ticket(&id) {
                Some(_) => {
                    println!("Deleted ticket with id: {}", id);
                }
                None => {
                    println!("Ticket with id {} not found", id);
                }
            }
        }
        None => {
            println!("No command provided");
        }
    }
}
