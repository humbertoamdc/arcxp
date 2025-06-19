use crate::requests::UpdateTicketStatusRequest;
use crate::tickets_repository::TicketRepository;
use anyhow::Result;

pub struct UpdateTicketStatus {
    tickets_repository: TicketRepository,
}

impl UpdateTicketStatus {
    pub fn new(tickets_repository: TicketRepository) -> Self {
        Self { tickets_repository }
    }
    pub async fn execute(&self, request: UpdateTicketStatusRequest) -> Result<()> {
        self.tickets_repository
            .update(request.id, None, None, None, Some(request.status), None)
            .await
    }
}
