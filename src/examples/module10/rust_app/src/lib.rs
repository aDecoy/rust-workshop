mod core;
mod data_access;

pub use crate::core::ApplicationError;

use anyhow::Result;
use crate::core::{DataAccess, LoginRequest, RegisterUserRequest, User, UserDetails};
use crate::data_access::PostgresUsers;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use core::Config;

pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess
}

pub async fn start() -> Result<(), ApplicationError> {
    let config = Config::get_configuration()?;

    let postgres_data_access = PostgresUsers::new(config.connection_string()).await?;
    
    let shared_state = Arc::new(AppState{
        data_access: postgres_data_access
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
    // insert your application logic here
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    match user {
        Ok(user) => {
            let data_access = state.data_access.store(user.clone()).await;

            match data_access {
                Ok(_) => (StatusCode::CREATED, Json(Some(user.details().clone()))),
                Err(e) => match e {
                    ApplicationError::UserDoesNotExist => {
                        (StatusCode::NOT_FOUND, Json(None))
                    },
                    _ => {
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
                    }
                } 
            }
        },
        Err(e) => {
            match e {
                ApplicationError::UserDoesNotExist => {
                    (StatusCode::NOT_FOUND, Json(None))
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
                }
            }
        }
    }
}

async fn login<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<UserDetails>>) {
    let user = state.data_access.with_email_address(&payload.email_address).await;
    
    match user { 
        Ok(user) =>{
            match user.verify_password(&payload.password) {
                Ok(_) => (StatusCode::OK, Json(Some(user.details().clone()))),
                Err(_) => (StatusCode::UNAUTHORIZED, Json(None)),
            }
        },
        Err(e) => {
            match e { 
                ApplicationError::UserDoesNotExist => {
                    (StatusCode::NOT_FOUND, Json(None))
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
                }
            } 
        }
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
            ApplicationError::UserDoesNotExist => {
                (StatusCode::NOT_FOUND, Json(None))
            },
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use crate::core::{ApplicationError, User};
    use std::sync::Arc;
    use mockall::mock;

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
        async fn with_email_address(&self, email_address: &str) -> std::result::Result<User, ApplicationError> {
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
            data_access: mock_data_access
        });

        let (status, response) = register_user(
            State(shared_state),
            Json(RegisterUserRequest {
                email_address: "test@test.com".to_string(),
                name: "Test User".to_string(),
                password: "Testing!23".to_string(),
            }),
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_user_with_mock_all() {
        let mut mock_data_access = MockDataAccess::new();
        mock_data_access
            .expect_store()
            .withf(|user| {
                user.email_address() == "test@test.com".to_string()
            })
            .return_once(move |_| Ok(()));
        let shared_state = Arc::new(AppState {
            data_access: mock_data_access
        });

        let (status, response) = register_user(
            State(shared_state),
            Json(RegisterUserRequest {
                email_address: "test@test.com".to_string(),
                name: "Test User".to_string(),
                password: "Testing!23".to_string(),
            }),
        ).await;

        assert_eq!(status, StatusCode::CREATED);
    }
}