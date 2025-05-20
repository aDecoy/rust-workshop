use std::sync::Arc;
use axum::extract::Path;
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Holds shared state to be used by the entire application.
#[derive(Default)]
struct AppState {
    /// All the users that are registered in the application.
    users: Vec<User>,
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        // When the /users route is hit, and it'a post request.
        // Call the register_user function
        .route("/users", post(register_user))
        .route("/users/{email_address}", get(get_user_details))
        State(state): State<SharedState>,

    // Create a TCP listener on port 3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // And run the app using the TCP listener
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn register_user(
    State(state): State<SharedState>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) // And the function returns a tuple StatusCode and body
{
    let user = User::new(&payload.email_address, &payload.name, &payload.password);

    state.write().await.users.push(user.clone());

    (StatusCode::CREATED, Json(user.details().clone()))
}

async fn get_user_details(
    State(state): State<SharedState>,
    Path(email_address): Path<String>,
) -> (StatusCode, Json<Option<UserDetails>>) // And the function returns a tuple StatusCode and body
{
    let users = &state.read().await.users;
    let user = users
        .iter()
        .find(|user| user.details().email_address == email_address);
    
    match user {
        Some(user) => (StatusCode::OK, Json(Some(user.details().clone()))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

#[derive(Deserialize)]
// Force camel case as Rust semantics use snake case by default
#[serde(rename_all = "camelCase")]
struct RegisterUserRequest {
    email_address: String,
    password: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    email_address: String,
    password: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UserDetails {
    email_address: String,
    password: String,
    age: Option<i32>,
    name: String,
}

#[derive(Clone)]
enum User {
    Standard {
        user_details: UserDetails,
    },
    Premium {
        user_details: UserDetails,
        is_premium: bool,
    },
}

impl User {
    fn details(&self) -> &UserDetails {
        match self {
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        }
    }

    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_name(&mut self, new_name: &str) {
        let mut user_details = match self {
            // The '*' is used to dereference the value of the variable, so you can change it.
            // De-referncing refers to accessing the underlying value the reference points to
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        };

        user_details.name = new_name.to_string();
    }

    fn update_age(&mut self, new_age: i32) {
        let mut user_details = match self {
            // The '*' is used to dereference the value of the variable, so you can change it.
            // De-referncing refers to accessing the underlying value the reference points to
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        };

        user_details.age = Some(new_age);
    }

    // &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
    fn say_hello(&self) {
        let name = match &self {
            User::Standard { user_details } => user_details.name.as_str(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.name.as_str(),
        };

        // String interpolation
        println!("Hello! I'm {}. I'm a standard user.", name);
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn get_age(&self) -> Option<i32> {
        let age = match &self {
            User::Standard { user_details } => user_details.age,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.age,
        };

        age
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn whats_my_age(&self) {
        // Everything in Rust returns a value, so you can assign a variable to the result of a match
        let users_age = match &self {
            User::Standard { user_details } => user_details.age,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.age,
        };

        // If let allows you to assign a variable and have an if condition in a single line
        if let Some(age) = users_age {
            println!("I'm {} years old.", age);
        } else {
            println!("I don't know my age.");
        }
    }

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> User {
        match self {
            User::Standard { user_details } => User::Premium {
                user_details,
                is_premium: true,
            },
            User::Premium { .. } => self,
        }
    }

    /// Create a new user with the given email address, name and password
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new("dev@jameseastham.co.uk", "James", "password");
    /// ```
    fn new(email_address: &str, name: &str, password: &str) -> User {
        User::Standard {
            user_details: UserDetails {
                email_address: email_address.to_string(),
                name: name.to_string(),
                age: None,
                password: password.to_string(),
            },
        }
    }
}
