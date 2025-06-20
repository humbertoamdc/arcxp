#[path = "common/mod.rs"]
mod common;

use crate::common::common::init_test_server;
use arcxp::entities::{Ticket, TicketPriority, TicketStatus};
use arcxp::requests::*;
use axum::http::StatusCode;
use serial_test::serial;

#[cfg(test)]
#[tokio::test]
#[serial]
async fn it_should_create_ticket_successfully() {
    let (server, app_state) = init_test_server().await;

    app_state.tickets_repository.clear_tickets_table().await;

    let payload = CreateTicketRequest {
        title: Some("Test ticket".to_string()),
        description: Some("Test description".to_string()),
        priority: Some(TicketPriority::P0),
    };

    let response = server.post("/tickets").json(&payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let tickets = query_test_tickets().await;
    assert_eq!(tickets.len(), 1);
    assert_eq!(tickets[0].title, Some("Test ticket".to_string()));
}

#[tokio::test]
#[serial]
async fn it_should_batch_create_tickets() {
    let (server, app_state) = init_test_server().await;

    app_state.tickets_repository.clear_tickets_table().await;

    let payload = BatchCreateTicketsRequest {
        tickets_data: vec![
            BatchCreateTicketsData {
                title: Some("Batch Ticket 1".to_string()),
                description: None,
                priority: Some(TicketPriority::P0),
            },
            BatchCreateTicketsData {
                title: Some("Batch Ticket 2".to_string()),
                description: Some("With description".to_string()),
                priority: Some(TicketPriority::P1),
            },
        ],
    };

    let response = server.post("/tickets/batch").json(&payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let tickets = query_test_tickets().await;
    assert_eq!(tickets.len(), 2);
}

#[tokio::test]
#[serial]
async fn it_should_update_ticket_status() {
    let (server, app_state) = init_test_server().await;

    app_state.tickets_repository.clear_tickets_table().await;

    let create_payload = CreateTicketRequest {
        title: Some("Test ticket".to_string()),
        description: Some("Test description".to_string()),
        priority: Some(TicketPriority::P0),
    };

    let response = server.post("/tickets").json(&create_payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let ticket = query_test_tickets().await[0].clone();
    let update_status_payload = UpdateTicketStatusRequest {
        id: ticket.id,
        status: TicketStatus::New,
    };

    let response = server
        .put("/tickets/status")
        .json(&update_status_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn it_should_assign_ticket() {
    let (server, app_state) = init_test_server().await;

    app_state.tickets_repository.clear_tickets_table().await;

    let create_payload = CreateTicketRequest {
        title: Some("Test ticket".to_string()),
        description: Some("Test description".to_string()),
        priority: Some(TicketPriority::P0),
    };

    let response = server.post("/tickets").json(&create_payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let ticket = query_test_tickets().await[0].clone();
    let assign_ticket_payload = AssignTicketRequest {
        ticket_id: ticket.id,
        assignee_id: "engineer-id-1".to_string(),
    };

    let response = server
        .put("/tickets/assignee")
        .json(&assign_ticket_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn it_should_fail_to_update_ticket_status_for_nonexistent_ticket_id() {
    let (server, app_state) = init_test_server().await;

    app_state.tickets_repository.clear_tickets_table().await;

    let update_status_payload = UpdateTicketStatusRequest {
        id: "nonexistent_ticket_id".to_string(),
        status: TicketStatus::New,
    };

    let response = server
        .put("/tickets/status")
        .json(&update_status_payload)
        .expect_failure()
        .await;
    assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

async fn query_test_tickets() -> Vec<Ticket> {
    let (server, _) = init_test_server().await;

    let response = server.get("/tickets").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let values = response.json::<Vec<serde_json::Value>>();

    values
        .into_iter()
        .map(|value| serde_json::from_value::<Ticket>(value).expect("failed to parse ticket"))
        .collect()
}
