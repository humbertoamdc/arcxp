use arcxp::app_state::AppState;
use arcxp::controllers::{
    assign_ticket, batch_create_tickets, create_ticket, query_tickets, update_ticket_status,
};
use axum::routing::{get, post, put};
use axum::Router;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = AppState::new().await;
    let app = Router::new()
        .route("/tickets", get(query_tickets))
        .route("/tickets", post(create_ticket))
        .route("/tickets/status", put(update_ticket_status))
        .route("/tickets/assignee", put(assign_ticket))
        .route("/tickets/batch", post(batch_create_tickets))
        .with_state(app_state);

    let deploy_target = env::var("DEPLOY_TARGET").unwrap_or(String::from("local"));

    match deploy_target.as_str() {
        "local" => {
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        "lambda" => {
            let lambda_app = tower::ServiceBuilder::new()
                .layer(axum_aws_lambda::LambdaLayer::default())
                .service(app);

            let _ = lambda_http::run(lambda_app).await;
        }
        _ => (),
    }
}
