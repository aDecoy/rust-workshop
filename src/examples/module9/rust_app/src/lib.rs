mod core;
mod data_access;

pub use crate::core::ApplicationError;

use crate::core::{
    DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails,
};
use crate::data_access::PostgresUsers;
use anyhow::Result;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Json, Router};
use core::Config;
use std::sync::Arc;

pub struct AppState<TDataAccess: DataAccess> {
    pub data_access: TDataAccess,
}

pub async fn start() -> Result<(), ApplicationError> {
    let config = Config::get_configuration()?;

    let postgres_data_access = PostgresUsers::new(config.connection_string()).await?;

    let shared_state = Arc::new(AppState {
        data_access: postgres_data_access,
    });

    let app = Router::new()
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        .with_state(shared_state);

    println!("Listening on port {}", config.app_port());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.app_port()))
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    Ok(())
}

async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    match user {
        Ok(user) => {
            let data_access = state.data_access.store(user.clone()).await;

            match data_access {
                Ok(_) => (StatusCode::CREATED, Json(Some(user.details().clone()))),
                Err(e) => match e {
                    // match on the application error to return the correct status code
                    ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
                    // If it's any other error code then return a 500
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
                },
            }
        }
        Err(e) => match e {
            ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
        },
    }
}

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
        Err(e) => match e {
            ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
        },
    }
}

async fn get_user_details<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state.data_access.with_email_address(&email_address).await;

    match user {
        Ok(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        Err(e) => match e {
            ApplicationError::UserDoesNotExist => (StatusCode::NOT_FOUND, Json(None)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
        },
    }
}
