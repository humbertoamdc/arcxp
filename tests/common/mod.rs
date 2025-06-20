pub mod common {
    use arcxp::app_state::AppState;
    use arcxp::controllers::*;
    use axum::{
        routing::{get, post, put},
        Router,
    };
    use axum_test::TestServer;

    pub async fn init_test_server() -> (TestServer, AppState) {
        std::env::set_var("AWS_ENDPOINT_URL", "http://127.0.0.1:4576");

        let app_state = AppState::new().await;

        let app = Router::new()
            .route("/tickets", post(create_ticket))
            .route("/tickets", get(query_tickets))
            .route("/tickets/status", put(update_ticket_status))
            .route("/tickets/assignee", put(assign_ticket))
            .route("/tickets/batch", post(batch_create_tickets))
            .with_state(app_state.clone());

        let test_server = TestServer::builder()
            .expect_success_by_default()
            .mock_transport()
            .build(app)
            .unwrap();

        (test_server, app_state)
    }
}
