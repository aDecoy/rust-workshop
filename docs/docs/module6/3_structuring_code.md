---
sidebar_position: 3
---

# Structuring Code Base

Look at how you can restructure your application using modules and the Ports and Adapters architecture:

## Step 1: Identify Your Core Domain

First, identify the core business logic of your application. In your case, it's the user management logic:

```rust showLineNumbers
// core.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
    pub email_address: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    email_address: String,
    password: String,
    age: Option<i32>,
    name: String,
}

#[derive(Clone)]
pub enum User {
    Standard { user_details: UserDetails },
    Premium { user_details: UserDetails, is_premium: bool }
}

impl User {
    pub fn new(email_address: &str, name: &str, password: &str) -> User {
        // Implementation
    }
    
    pub fn details(&self) -> &UserDetails {
        // Implementation
    }
    
    // Other methods.
}
```

Note the use of `pub` for items that need to be accessible from other modules.

## Step 2: Define Data Access Layer

Next, create a separate module for data access concerns:

```rust showLineNumbers
// data_access.rs
use std::sync::{Arc, RwLock};
use crate::core::User;

#[derive(Default)]
pub struct AppState {
    pub(crate) users: Vec<User>,
}

pub type SharedState = Arc<RwLock<AppState>>;
```

The `pub(crate)` modifier makes `users` accessible only within the current crate, not to external users of the library.

## Step 3: Wire Everything in the Main Module

Finally, use both modules in your main application:

```rust showLineNumbers
// main.rs
mod core;
mod data_access;

use crate::core::{LoginRequest, RegisterUserRequest, User, UserDetails};
use crate::data_access::SharedState;
use axum::{/* . */};

#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        .route("/users", post(register_user))
        // Other routes.
        .layer(Extension(SharedState::default()));

    // Start the server.
}

async fn register_user(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    state.write().unwrap().users.push(user.clone());
    
    (StatusCode::CREATED, Json(user.details().clone()))
}

// Other handler functions.
```

:::info

Note the use of `use crate::core::{LoginRequest, RegisterUserRequest, User, UserDetails};` in the imports at the top of the file. `crate::` is a special import syntax to import things from over crates in the same package

:::

## The Benefits of This Structure

This modular approach offers you several benefits:

1. **Separation of Concerns**: Business logic is isolated from infrastructure details
2. **Testability**: Core domain logic can be tested independently
3. **Flexibility**: External systems can be replaced without changing the core
4. **Maintainability**: Clear boundaries make the codebase easier to understand
5. **Reusability**: Core logic can be reused across different interfaces

## Comparing with .NET

| Concept | .NET | Rust |
|---------|------|------|
| Organizing Code | Namespaces, Assemblies | Modules, Crates |
| Access Control | public, internal, private | pub, pub(crate), no modifier |
| Project Structure | .csproj files | Cargo.toml |
| Interface Definition | interfaces | traits |
| Dependency Inversion | DI Container | Trait objects, Generics |