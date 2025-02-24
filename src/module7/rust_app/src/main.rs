mod core;
mod data_access;

use std::sync::{Arc, Mutex};
use crate::core::{DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails};
use axum::extract::{Path, State};
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use crate::data_access::InMemoryDataAccess;

pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState{
        data_access: InMemoryDataAccess::new()
    });
    
    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `register_user`
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    State(mut state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // insert your application logic here
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    state.data_access.store(user.clone());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
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
