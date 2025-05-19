use log::info;
use rust_users_lib::{init_tracing_subscriber, ApplicationError};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    info!("Starting the application");

    rust_users_lib::init_logger();
    let _otel_guard = init_tracing_subscriber();

    tokio::spawn(async move { rust_users_lib::start_background_worker().await });

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutting down");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    };

    Ok(())
}
