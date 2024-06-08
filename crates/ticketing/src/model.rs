use super::prelude::*;
use chrono::{DateTime, Utc};
use cli_table::Table;
use core::str;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

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

pub struct TicketBuilder {
    id: TicketId,
    key: String,
    title: TicketTitle,
    status: Status,
    description: TicketDescription,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TicketBuilder {
    pub fn new(id: TicketId, key: String) -> Self {
        Self {
            id,
            key,
            title: TicketTitle("".to_string()),
            status: Status::ToDo,
            description: TicketDescription("".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn from_ticket(ticket: &Ticket) -> Self {
        Self {
            id: ticket.id,
            key: ticket.key.clone(),
            title: ticket.title.clone(),
            status: ticket.status.clone(),
            description: ticket.description.clone(),
            created_at: ticket.created_at.clone(),
            updated_at: ticket.updated_at.clone(),
        }
    }

    pub fn title(mut self, title: TicketTitle) -> Self {
        self.title = title;
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn description(mut self, description: TicketDescription) -> Self {
        self.description = description;
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn build(self) -> Ticket {
        Ticket {
            id: self.id,
            key: self.key,
            title: self.title,
            status: self.status,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
