---
sidebar_position: 1
---

# Module 6: Structuring Your Rust Application - Modules and Architecture

In this module, you'll learn how to structure larger Rust applications using modules and the Ports and Adapters architecture pattern (also known as Hexagonal Architecture). For you as a .NET developer, this is comparable to organizing your solution with Clean Architecture principles, separating core business logic from infrastructure concerns.

## Learning Objectives

By the end of this module, you will:
- Understand the Ports and Adapters (Hexagonal) architecture pattern
- Learn how to use Rust's module system to organize your code
- Implement a clean separation between business logic and data access
- Apply access control modifiers to enforce architectural boundaries

## Understanding Ports and Adapters Architecture

Ports and Adapters, or Hexagonal Architecture, is a software architecture pattern that emphasizes the separation of concerns by dividing your application into layers:

1. **Core Domain**: Contains your business logic and domain entities
2. **Ports**: Define interfaces that the core uses to communicate with the outside world
3. **Adapters**: Implement the interfaces defined by ports, connecting external systems to your core

This pattern allows you to isolate your core business logic from external dependencies, making it more testable and maintainable.

For you as a .NET developer, this is similar to Clean Architecture or Onion Architecture, where dependencies point inward, and the core is independent of infrastructure concerns.

## Rust's Module System

Rust provides a module system that allows you to organize code into logical units and control visibility. Unlike namespaces in C#, Rust modules also control item visibility through access modifiers.

### Declaring Modules

In Rust, you can declare modules in two ways:

1. **In-file module declarations**:

```rust
mod core {
    // Module contents
}
```

2. **Separate files**:

```rust
// In main.rs
mod core; // Tells Rust to look for core.rs or core/mod.rs
```

The second approach is more common for larger applications and is what you'll use in this module.

### Module Visibility

Rust has a powerful system of access modifiers:

- `pub`: Public, visible everywhere
- No modifier: Private to the current module
- `pub(crate)`: Visible only within the current crate
- `pub(super)`: Visible to the parent module
- `pub(in path)`: Visible to a specific path

These modifiers help you enforce architectural boundaries.

## Implementing the Architecture

Let's look at how you can restructure your application using modules and the Ports and Adapters architecture:

### Step 1: Identify Your Core Domain

First, identify the core business logic of your application. In your case, it's the user management logic:

```rust
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
    
    // Other methods...
}
```

Note the use of `pub` for items that need to be accessible from other modules.

### Step 2: Define Data Access Layer

Next, create a separate module for data access concerns:

```rust
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

### Step 3: Wire Everything in the Main Module

Finally, use both modules in your main application:

```rust
// main.rs
mod core;
mod data_access;

use crate::core::{LoginRequest, RegisterUserRequest, User, UserDetails};
use crate::data_access::SharedState;
use axum::{/* ... */};

#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        .route("/users", post(register_user))
        // Other routes...
        .layer(Extension(SharedState::default()));

    // Start the server...
}

async fn register_user(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    state.write().unwrap().users.push(user.clone());
    
    (StatusCode::CREATED, Json(user.details().clone()))
}

// Other handler functions...
```

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

## Your Challenge

Now it's time to apply what you've learned about modular architecture! In this module's challenge, you'll:

1. Take the monolithic web API from the previous module and split it into a modular architecture
2. Create separate files for:
   - `core.rs`: Containing your domain models and business logic
   - `data_access.rs`: Containing your data storage mechanism
   - `main.rs`: For your API endpoints and application setup
3. Apply appropriate visibility modifiers to enforce architectural boundaries
4. Ensure all modules are properly connected and the API still works as expected

Specifically, you need to:
- Move all domain types (User, UserDetails) and their implementations to the core module
- Move data storage (AppState) to the data access module
- Use proper visibility modifiers (pub, pub(crate)) to restrict access where appropriate
- Create clear module boundaries between business logic and infrastructure concerns
- Update imports in the main file to reference these new modules

The starter code is available in `src/module6/rust_app`, and you can check your solution against `src/module6/rust_app_final`.

Good luck, and remember that a well-structured application will be much easier to maintain and extend as it grows!