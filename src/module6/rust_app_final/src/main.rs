// Tell the compiler that there is another module in this 
// application. By convention the name of the file should match
// the name of the module
mod core;
mod data_access;

use crate::core::{LoginRequest, RegisterUserRequest, User, UserDetails};
use axum::extract::Path;
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum::http::header::AGE;
use crate::data_access::SharedState;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `register_user`
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        .layer(Extension(SharedState::default()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn register_user(
    Extension(state): Extension<SharedState>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // insert your application logic here
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    state.write().unwrap().users.push(user.clone());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user.details().clone()))
}

async fn login(
    Extension(state): Extension<SharedState>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let users = &state.read().unwrap().users;
    let user = users
        .iter()
        .find(|user| user.email_address() == payload.email_address);

    println!("{:?}", user.is_some());

    if let Some(user) = user {
        return match user.verify_password(&payload.password) {
            Ok(_) => (StatusCode::OK, Json(Some(user.details().clone()))),
            Err(_) => (StatusCode::UNAUTHORIZED, Json(None)),
        };
    }

    (StatusCode::NOT_FOUND, Json(None))
}

async fn get_user_details(
    Extension(state): Extension<SharedState>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let users = &state.read().unwrap().users;
    let user = users
        .iter()
        .find(|user| user.email_address() == email_address);

    match user {
        Some(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
