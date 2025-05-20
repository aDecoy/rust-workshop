use rust_users_lib::ApplicationError;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    rust_users_lib::start().await
}
