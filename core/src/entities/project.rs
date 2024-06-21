use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: u32,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(key: String, name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            key,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}
