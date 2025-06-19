use crate::entities::Ticket;
use crate::tickets_repository::TicketRepository;
use anyhow::Result;

pub struct QueryTickets {
    tickets_repository: TicketRepository,
}

impl QueryTickets {
    pub fn new(tickets_repository: TicketRepository) -> Self {
        Self { tickets_repository }
    }
    pub async fn execute(&self) -> Result<Vec<Ticket>> {
        self.tickets_repository.query().await
    }
}
