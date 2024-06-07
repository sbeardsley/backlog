use crate::domain::{
    DeletedTicket, Ticket, TicketDescription, TicketDraft, TicketId, TicketPatch, TicketStore,
    TicketTitle,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
pub struct InitRequiredError;

impl Error for InitRequiredError {}

impl std::fmt::Display for InitRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", "Backlog not found, run `backlog init`")
    }
}

#[derive(Serialize, Deserialize)]
pub struct BacklogApp {
    name: String,
    key: String,
    path: String,
    store: TicketStore,
}

fn generate_ticket_draft(
    title: TicketTitle,
    description: Option<TicketDescription>,
) -> TicketDraft {
    TicketDraft { title, description }
}

impl BacklogApp {
    pub fn new() -> Result<BacklogApp, InitRequiredError> {
        let backlog_path = Path::new(".backlog.yaml");
        Self::load(backlog_path)
    }

    pub fn init(name: String, key: String) {
        let filename = format!(".backlog.yaml");
        let backlog_path = Path::new(&filename);
        match Self::load(backlog_path) {
            Ok(_) => {
                println!("Backlog already exists.");
            }
            Err(InitRequiredError) => {
                let store = TicketStore::new(key.clone());
                let app = BacklogApp {
                    name,
                    store,
                    key,
                    path: filename.to_string(),
                };
                app.save();
            }
        }
    }

    pub fn create_ticket(
        &mut self,
        title: TicketTitle,
        description: Option<TicketDescription>,
    ) -> TicketId {
        let ticket_draft = generate_ticket_draft(title, description);
        let ticket_id = self.store.save(ticket_draft);
        self.save();
        ticket_id
    }

    pub fn list_tickets(&self) -> Vec<&Ticket> {
        self.store.list()
    }

    pub fn update_ticket(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
        self.store.update(id, patch);
        self.save();
        self.store.get(id)
    }

    pub fn delete_ticket(&mut self, id: &TicketId) -> Option<DeletedTicket> {
        let ticket = self.store.delete(id);
        self.save();
        ticket
    }

    // Load backlog from disk.
    fn load(path: &Path) -> Result<BacklogApp, InitRequiredError> {
        // Read the data in memory, storing the value in a string
        match read_to_string(path) {
            Ok(data) => {
                // Deserialize configuration from YAML format
                Ok(serde_yaml::from_str(&data).expect("Failed to parse serialised data."))
            }
            Err(e) => match e.kind() {
                // The file is missing - this is the first time you are using backlog!
                std::io::ErrorKind::NotFound => {
                    // Return default configuration
                    Err(InitRequiredError)
                }
                // Something went wrong - crash the CLI with an error message.
                _ => panic!("Failed to read data."),
            },
        }
    }

    /// Save tickets on disk in the right file.
    fn save(&self) {
        let path = Path::new(&self.path);
        // Serialize data to YAML format
        let content = serde_yaml::to_string(self).expect("Failed to serialize backlog");
        println!("Saving backlog to {:?}", path);
        // Save to disk
        std::fs::write(path, content).expect("Failed to write backlog to disk.")
    }
}
