use log::info;
use rust_users_lib::{init_tracing_subscriber, ApplicationError};

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    info!("Starting the application");

    rust_users_lib::init_logger();
    let _otel_guard = init_tracing_subscriber();

    rust_users_lib::start_api().await
}
