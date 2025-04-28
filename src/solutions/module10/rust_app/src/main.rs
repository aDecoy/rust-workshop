mod core;
mod data_access;

use crate::core::{
    ApplicationError, DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails,
};
use crate::data_access::PostgresUsers;
use anyhow::Result;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use tracing::info;

pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess,
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    tracing_subscriber::fmt().json().init();

    let postgres_data_access = PostgresUsers::new().await?;

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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    tracing::info!("Starting application on port 3000");

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

    Ok(())
}

async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    info!("Handling register user request");

    // insert your application logic here
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    match user {
        Ok(user) => {
            let data_access = state.data_access.store(user.clone()).await;

            match data_access {
                Ok(_) => (StatusCode::CREATED, Json(Some(user.details().clone()))),
                Err(e) => match e {
                    ApplicationError::UserDoesNotExist => {
                        tracing::error!("{}", e);
                        (StatusCode::NOT_FOUND, Json(None))
                    }
                    _ => {
                        tracing::error!("{}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
                    }
                },
            }
        }
        Err(e) => match e {
            ApplicationError::UserDoesNotExist => {
                tracing::error!("{}", e);
                (StatusCode::NOT_FOUND, Json(None))
            }
            _ => {
                tracing::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        },
    }
}

async fn login<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    info!("Handling login request");
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
            ApplicationError::UserDoesNotExist => {
                tracing::error!("{}", e);
                (StatusCode::NOT_FOUND, Json(None))
            }
            _ => {
                tracing::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        },
    }
}

async fn get_user_details<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    info!("Handling get user details request");

    let user = state.data_access.with_email_address(&email_address).await;

    match user {
        Ok(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        Err(e) => match e {
            ApplicationError::UserDoesNotExist => {
                tracing::error!("{}", e);
                (StatusCode::NOT_FOUND, Json(None))
            }
            _ => {
                tracing::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        },
    }
}
