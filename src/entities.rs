use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TicketStatus {
    New,
    Triaging,
    InProgress,
    InReview,
    Done,
    Close,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TicketPriority {
    P0,
    P1,
    P2,
    P3,
    P4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticket {
    pub id: String,
    pub assignee_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: Option<TicketPriority>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn new(
        title: Option<String>,
        description: Option<String>,
        status: Option<TicketStatus>,
        priority: Option<TicketPriority>,
    ) -> Self {
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new())).to_string();

        let now = Utc::now();

        Self {
            id,
            assignee_id: None,
            title,
            description,
            status: status.unwrap_or(TicketStatus::New),
            priority,
            created_at: now,
            updated_at: now,
        }
    }
}
