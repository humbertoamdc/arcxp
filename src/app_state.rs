use crate::tickets_repository::TicketRepository;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::config::BehaviorVersion;

#[derive(Clone)]
pub struct AppState {
    pub tickets_repository: TicketRepository,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            tickets_repository: TicketRepository::new(
                dynamodb_client().await,
                String::from("Tickets"),
            ),
        }
    }
}

async fn dynamodb_client() -> aws_sdk_dynamodb::Client {
    let shared_config = get_shared_config().await;
    let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

    aws_sdk_dynamodb::Client::from_conf(dynamodb_config)
}

async fn get_shared_config() -> SdkConfig {
    let mut shared_config = aws_config::defaults(BehaviorVersion::latest());
    let env = std::env::var("ENV").unwrap_or(String::from("development"));
    if env == String::from("development") {
        shared_config = shared_config.endpoint_url(std::env::var("AWS_ENDPOINT_URL").unwrap());
    }
    shared_config.load().await
}
