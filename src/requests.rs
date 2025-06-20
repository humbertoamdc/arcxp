use crate::entities::{TicketPriority, TicketStatus};
use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateTicketStatusRequest {
    pub id: String,
    pub status: TicketStatus,
}
#[derive(Serialize, Deserialize)]
pub struct AssignTicketRequest {
    pub ticket_id: String,
    pub assignee_id: String,
}
#[derive(Serialize, Deserialize)]
pub struct BatchCreateTicketsRequest {
    pub tickets_data: Vec<BatchCreateTicketsData>,
}
#[derive(Serialize, Deserialize)]
pub struct BatchCreateTicketsData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}
