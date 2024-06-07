use chrono::{DateTime, Utc};
use cli_table::Table;
use core::str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ParsingError(String);
impl Error for ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_id: u128,
    key: String,
}

impl TicketStore {
    pub fn new(key: String) -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_id: 0,
            key,
        }
    }

    pub fn save(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();
        let temp_description: TicketDescription;

        if let Some(description) = draft.description {
            temp_description = description;
        } else {
            temp_description = TicketDescription::default();
        }

        let now = Utc::now();
        let ticket = Ticket {
            id,
            status: Status::ToDo,
            key: format!("{}-{}", self.key, id),
            title: draft.title,
            description: temp_description,
            created_at: now.clone(),
            updated_at: now,
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.values().collect()
    }

    pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
        if let Some(ticket) = self.data.get_mut(id) {
            if let Some(title) = patch.title {
                ticket.title = title;
            }
            if let Some(description) = patch.description {
                ticket.description = description;
            }
            if let Some(status) = patch.status {
                ticket.status = status;
            }
            ticket.updated_at = Utc::now();
            Some(ticket)
        } else {
            None
        }
    }
    pub fn delete(&mut self, id: &TicketId) -> Option<DeletedTicket> {
        self.data.remove(id).map(|ticket| DeletedTicket {
            ticket,
            deleted_at: Utc::now(),
        })
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Result<Self, ValidationError> {
        if title.is_empty() {
            return Err(ValidationError(
                "Title is not a valid description".to_string(),
            ));
        }
        if title.len() > 255 {
            return Err(ValidationError(
                "Title cannot be longer than 255 characters".to_string(),
            ));
        }
        Ok(Self(title))
    }
}

impl Display for TicketTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TicketTitle {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TicketTitle::new(s.to_string()) {
            Ok(title) => Ok(title),
            Err(err) => match err {
                ValidationError(_) => Err(ParsingError(format!("{} is not a valid title", s))),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn new(description: String) -> Result<Self, ValidationError> {
        if description.len() > 3000 {
            return Err(ValidationError(
                "Description cannot be longer than 3000 characters".to_string(),
            ));
        }
        Ok(Self(description))
    }

    pub fn default() -> TicketDescription {
        TicketDescription("".to_string())
    }
}

impl Display for TicketDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TicketDescription {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TicketDescription::new(s.to_string()) {
            Ok(description) => Ok(description),
            Err(err) => match err {
                ValidationError(_) => {
                    Err(ParsingError(format!("{} is not a valid description", s)))
                }
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Status::ToDo => write!(f, "To Do"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Blocked => write!(f, "Blocked"),
            Status::Done => write!(f, "Done"),
        }
    }
}

impl FromStr for Status {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Status::ToDo),
            "InProgress" => Ok(Status::InProgress),
            "Blocked" => Ok(Status::Blocked),
            "Done" => Ok(Status::Done),
            _ => Err(ParsingError(format!("{} is not a valid status", s))),
        }
    }
}

pub type TicketId = u128;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    id: TicketId,
    key: String,
    title: TicketTitle,
    status: Status,
    description: TicketDescription,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn title(&self) -> &TicketTitle {
        &self.title
    }
    pub fn description(&self) -> &TicketDescription {
        &self.description
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn id(&self) -> &TicketId {
        &self.id
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn to_table_row(&self) -> TicketRow {
        TicketRow {
            id: self.id,
            key: self.key.clone(),
            title: self.title.clone().0,
            status: self.status.clone(),
            description: self.description.clone().0,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: Option<TicketDescription>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

pub struct DeletedTicket {
    pub ticket: Ticket,
    pub deleted_at: DateTime<Utc>,
}
