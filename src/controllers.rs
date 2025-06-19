use crate::app_state::AppState;
use crate::requests::{
    AssignTicketRequest, BatchCreateTicketsRequest, CreateTicketRequest, UpdateTicketStatusRequest,
};
use crate::usecases::assign_ticket::AssignTicket;
use crate::usecases::batch_create_tickets::BatchCreateTicket;
use crate::usecases::create_ticket::CreateTicket;
use crate::usecases::query_tickets::QueryTickets;
use crate::usecases::update_ticket_status::UpdateTicketStatus;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use lambda_http::tracing;

pub async fn query_tickets(State(app_state): State<AppState>) -> impl IntoResponse {
    let usecase = QueryTickets::new(app_state.tickets_repository);
    let result = usecase.execute().await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_ticket(
    State(app_state): State<AppState>,
    Json(request): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    let usecase = CreateTicket::new(app_state.tickets_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_ticket_status(
    State(app_state): State<AppState>,
    Json(request): Json<UpdateTicketStatusRequest>,
) -> impl IntoResponse {
    let usecase = UpdateTicketStatus::new(app_state.tickets_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn assign_ticket(
    State(app_state): State<AppState>,
    Json(request): Json<AssignTicketRequest>,
) -> impl IntoResponse {
    let usecase = AssignTicket::new(app_state.tickets_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn batch_create_tickets(
    State(app_state): State<AppState>,
    Json(request): Json<BatchCreateTicketsRequest>,
) -> impl IntoResponse {
    let usecase = BatchCreateTicket::new(app_state.tickets_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
