use crate::entities::{Ticket, TicketStatus};
use crate::requests::BatchCreateTicketsRequest;
use crate::tickets_repository::TicketRepository;
use anyhow::Result;

pub struct BatchCreateTicket {
    tickets_repository: TicketRepository,
}

impl BatchCreateTicket {
    pub fn new(tickets_repository: TicketRepository) -> Self {
        Self { tickets_repository }
    }
    pub async fn execute(&self, request: BatchCreateTicketsRequest) -> Result<()> {
        let tickets = request
            .tickets_data
            .into_iter()
            .map(|ticket_data| {
                Ticket::new(
                    ticket_data.title,
                    ticket_data.description,
                    Some(TicketStatus::New),
                    ticket_data.priority,
                )
            })
            .collect();

        self.tickets_repository.batch_create(tickets).await
    }
}
