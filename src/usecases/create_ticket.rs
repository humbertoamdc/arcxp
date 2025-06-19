use crate::entities::{Ticket, TicketStatus};
use crate::requests::CreateTicketRequest;
use crate::tickets_repository::TicketRepository;
use anyhow::Result;

pub struct CreateTicket {
    tickets_repository: TicketRepository,
}

impl CreateTicket {
    pub fn new(tickets_repository: TicketRepository) -> Self {
        Self { tickets_repository }
    }
    pub async fn execute(&self, request: CreateTicketRequest) -> Result<()> {
        let ticket = Ticket::new(
            request.title,
            request.description,
            Some(TicketStatus::New),
            request.priority,
        );

        self.tickets_repository.create(ticket).await
    }
}
