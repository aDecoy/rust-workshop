mod core;
mod data_access;

use crate::core::{DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails};
use crate::data_access::InMemoryDataAccess;
use axum::extract::{Path, State};
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use std::sync::{Arc, Mutex};

// Wrap all of our shared state/dependencies in a single struct
// Using a generic.
// This is saying the TDataAccess type should implement DataAccess and be thread-safe.
// Which we have by using the Arc data type
pub struct AppState<TDataAccess: DataAccess> {
    pub data_access: TDataAccess,
}

pub async fn start() {
    // Initialize a new instance of our application state on startup
    let app_state = AppState {
        data_access: InMemoryDataAccess::new(),
    };

    // Wrap our application state in an Arc, which is a thread-safe reference-counted pointer
    let shared_state = Arc::new(app_state);

    let app = Router::new()
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        // Add the shared state to our app using the with_state function
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// The register user function now has a generic type parameter TDataAccess.
// TDataAccess should be of type DataAccess (the custom trait)
// As well as send and sync, which are traits that allow the type to be sent between threads
// This is saying the TDataAccess type should implement DataAccess and be thread-safe. Which we have by using the Arc data type
async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    // Inject the state into the functions
    State(state): State<Arc<AppState<TDataAccess>>>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    state.data_access.store(user.clone());

    (StatusCode::CREATED, Json(user.details().clone()))
}

async fn login<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state.data_access.with_email_address(&payload.email_address);

    if let Some(user) = user {
        return match user.verify_password(&payload.password) {
            Ok(_) => (StatusCode::OK, Json(Some(user.details().clone()))),
            Err(_) => (StatusCode::UNAUTHORIZED, Json(None)),
        };
    }

    (StatusCode::NOT_FOUND, Json(None))
}

async fn get_user_details<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state.data_access.with_email_address(&email_address);

    match user {
        Some(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
