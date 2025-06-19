use crate::entities::{TicketPriority, TicketStatus};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}
#[derive(Deserialize)]
pub struct UpdateTicketStatusRequest {
    pub id: String,
    pub status: TicketStatus,
}
#[derive(Deserialize)]
pub struct AssignTicketRequest {
    pub ticket_id: String,
    pub assignee_id: String,
}
#[derive(Deserialize)]
pub struct BatchCreateTicketsRequest {
    pub tickets_data: Vec<BatchCreateTicketsData>,
}
#[derive(Deserialize)]
pub struct BatchCreateTicketsData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}
