use crate::requests::AssignTicketRequest;
use crate::tickets_repository::TicketRepository;
use anyhow::Result;

pub struct AssignTicket {
    tickets_repository: TicketRepository,
}

impl AssignTicket {
    pub fn new(tickets_repository: TicketRepository) -> Self {
        Self { tickets_repository }
    }
    pub async fn execute(&self, request: AssignTicketRequest) -> Result<()> {
        self.tickets_repository
            .update(
                request.ticket_id,
                Some(request.assignee_id),
                None,
                None,
                None,
                None,
            )
            .await
    }
}
