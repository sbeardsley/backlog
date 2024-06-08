use chrono::Utc;
use core::str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::{
    DeletedTicket, Status, Ticket, TicketBuilder, TicketDescription, TicketDraft, TicketId,
    TicketPatch,
};

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
        let ticket = TicketBuilder::new(id, format!("{}-{}", self.key, id))
            .title(draft.title)
            .description(temp_description)
            .status(Status::ToDo)
            .created_at(now.clone())
            .updated_at(now)
            .build();

        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.values().collect()
    }

    pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<Ticket> {
        let existing_ticket = self.data.get(id).unwrap();
        let builder = TicketBuilder::from_ticket(existing_ticket);

        let ticket = builder
            .title(patch.title.unwrap_or(existing_ticket.title().clone()))
            .description(
                patch
                    .description
                    .unwrap_or(existing_ticket.description().clone()),
            )
            .status(patch.status.unwrap_or(existing_ticket.status().clone()))
            .updated_at(Utc::now())
            .build();

        Some(ticket)
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
