mod core;
mod data_access;

pub use crate::core::ApplicationError;

use crate::core::{DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails};
use crate::data_access::PostgresUsers;
use anyhow::Result;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Json, Router};
use core::Config;
use log::info;
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, ConsumerContext};
use rdkafka::Message;
use std::sync::Arc;
use structured_logger::{async_json::new_writer, Builder};
use tracing::Level;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess,
}

pub fn init_logger() {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());

    // Initialize the logger.
    Builder::with_level(&log_level)
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init()
}

pub async fn start_background_worker() -> Result<(), ApplicationError> {
    let config = Config::get_configuration()?;

    let postgres_data_access = PostgresUsers::new(config.connection_string()).await?;

    let shared_state = Arc::new(AppState {
        data_access: postgres_data_access,
    });

    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", config.kafka_group_id())
        .set("bootstrap.servers", config.kafka_broker())
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    let channels = vec!["order-completed"];
    consumer
        .subscribe(&channels)
        .expect("Can't subscribe to specified topics");

    loop {
        // Perform some background task
        log::info!("Background worker is running...");
        match consumer.recv().await {
            Err(e) => tracing::warn!("Kafka error: {}", e),
            Ok(m) => {
                info!("Received message");
                info!("Message: {:?}", m.payload_view::<str>());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

pub async fn start_api() -> Result<(), ApplicationError> {
    let config = Config::get_configuration()?;

    let postgres_data_access = PostgresUsers::new(config.connection_string()).await?;

    let shared_state = Arc::new(AppState {
        data_access: postgres_data_access,
    });

    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `register_user`
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    println!("Listening on port {}", config.app_port());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.app_port()))
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    log::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    Ok(())
}

#[tracing::instrument(skip(state, payload), fields(user.email_is_valid, user.password_is_valid))]
async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    // insert your application logic here
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    match user {
        Ok(user) => {
            let data_access = state.data_access.store(user.clone()).await;

            match data_access {
                Ok(_) => (StatusCode::CREATED, Json(Some(user.details().clone()))),
                Err(e) => {
                    log::error!("{:?}", e);
                    match e {
                        ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
                        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
                    }
                }
            }
        }
        Err(e) => {
            log::error!("{:?}", e);
            match e {
                ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
    }
}

#[tracing::instrument(skip(state, payload))]
async fn login<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state
        .data_access
        .with_email_address(&payload.email_address)
        .await;

    match user {
        Ok(user) => match user.verify_password(&payload.password) {
            Ok(_) => (StatusCode::OK, Json(Some(user.details().clone()))),
            Err(_) => (StatusCode::UNAUTHORIZED, Json(None)),
        },
        Err(e) => {
            log::error!("{:?}", e);
            match e {
                ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
    }
}

#[tracing::instrument(skip(state, email_address))]
async fn get_user_details<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state.data_access.with_email_address(&email_address).await;

    match user {
        Ok(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        Err(e) => {
            log::error!("{:?}", e);
            match e {
                ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
    }
}

pub struct OtelGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::builder()
        .with_schema_url(
            [
                KeyValue::new(SERVICE_NAME, "users-service"),
                KeyValue::new(SERVICE_VERSION, "1.0.0"),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
            ],
            SCHEMA_URL,
        )
        .build()
}

// Construct TracerProvider for OpenTelemetryLayer
fn init_tracer_provider() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        // If export trace to AWS X-Ray, you can use XrayIdGenerator
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

// Initialize tracing-subscriber and return OtelGuard for opentelemetry-related termination processing
pub fn init_tracing_subscriber() -> OtelGuard {
    let tracer_provider = init_tracer_provider();

    let tracer = tracer_provider.tracer("users-service");

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    OtelGuard { tracer_provider }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ApplicationError, User};
    use mockall::mock;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Create a mock implementation for testing
    struct ManualMockDataAccess {
        // You can store expected results or track calls
        users: HashMap<String, User>,
    }

    impl ManualMockDataAccess {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }
    }

    mock! {
        DataAccess{}
        #[async_trait::async_trait]
        impl DataAccess for DataAccess {
            async fn with_email_address(&self, email_address: &str) -> std::result::Result<User, ApplicationError>;
            async fn store(&self, user: User) -> std::result::Result<(), ApplicationError>;
        }
    }

    #[async_trait::async_trait]
    impl DataAccess for ManualMockDataAccess {
        async fn with_email_address(
            &self,
            email_address: &str,
        ) -> std::result::Result<User, ApplicationError> {
            if let Some(user) = self.users.get(email_address) {
                Ok(user.clone())
            } else {
                Err(ApplicationError::UserDoesNotExist)
            }
        }

        async fn store(&self, user: User) -> std::result::Result<(), ApplicationError> {
            // Simulate storing the user
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_register_user_with_manual_mock() {
        let mock_data_access = ManualMockDataAccess::new();
        let shared_state = Arc::new(AppState {
            data_access: mock_data_access,
        });

        let (status, response) = register_user(
            State(shared_state),
            Json(RegisterUserRequest {
                email_address: "test@test.com".to_string(),
                name: "Test User".to_string(),
                password: "Testing!23".to_string(),
            }),
        )
        .await;

        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_user_with_mock_all() {
        let mut mock_data_access = MockDataAccess::new();
        mock_data_access
            .expect_store()
            .withf(|user| user.email_address() == "test@test.com".to_string())
            .return_once(move |_| Ok(()));
        let shared_state = Arc::new(AppState {
            data_access: mock_data_access,
        });

        let (status, response) = register_user(
            State(shared_state),
            Json(RegisterUserRequest {
                email_address: "test@test.com".to_string(),
                name: "Test User".to_string(),
                password: "Testing!23".to_string(),
            }),
        )
        .await;

        assert_eq!(status, StatusCode::CREATED);
    }
}
