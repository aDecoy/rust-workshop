---
sidebar_position: 1
---

# Module 5: Building Web APIs with Rust

In this module, you'll learn how to build web APIs in Rust using the Tokio async runtime and the Axum web framework. For you as a .NET developer, this is comparable to building web APIs with ASP.NET Core, but with Rust's performance, safety, and concurrency advantages.

## Learning Objectives

By the end of this module, you will:
- Understand what Tokio is and why it's essential for asynchronous programming in Rust
- Learn how to use the Axum web framework to build HTTP APIs
- Compare Rust's approach to web API development with .NET's approach
- Implement shared state and dependency injection patterns in Rust

## What is Tokio?

Tokio is a runtime for asynchronous programming in Rust. While .NET has built-in support for async/await, Rust's async functionality is provided by external crates. Tokio is the most popular async runtime and provides:

- A multi-threaded runtime for executing asynchronous code
- Utilities for asynchronous I/O operations
- Synchronization primitives for concurrent code
- Tools for working with time (delays, timeouts)

Tokio is essential for building network services because it allows your application to handle many connections concurrently without blocking.

### The #[tokio::main] Attribute

The `#[tokio::main]` attribute macro transforms your `main` function into one that initializes the Tokio runtime:

```rust showLineNumbers
#[tokio::main]
async fn main() {
    // Your asynchronous code here
}
```

Under the hood, this expands to code that creates a runtime and runs your async main function to completion.

## Introduction to Axum

Axum is a web framework built on top of Tokio. Created by the same team, it's designed to be:

- Modular and composable
- Type-safe and ergonomic
- Compatible with the broader Tokio ecosystem

For you as a .NET developer, Axum serves a similar role to ASP.NET Core, but with a more functional and composable approach.

### Key Components of Axum

1. **Routers**: Define the routes your API will handle
2. **Handlers**: Functions that process requests and return responses
3. **Extractors**: Types that extract data from requests
4. **Middleware**: Components that process requests before they reach handlers

## Building a Simple API with Axum

Let's examine how to build a basic API in Rust compared to .NET:

### Rust (with Axum):

```rust showLineNumbers
use axum::{
    routing::post,
    Router,
};

#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        .route("/users", post(register_user));

    // Start the server on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn register_user() -> &'static str {
    "User registered"
}
```

### .NET (with ASP.NET Core):

```csharp showLineNumbers
var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapPost("/users", () => "User registered");

app.Run("http://0.0.0.0:3000");
```

While both examples are concise, the approaches differ:

1. In Rust, routes are explicitly defined and associated with handler functions
2. In .NET, the minimal API approach uses method chaining with inline lambda expressions
3. Rust separates the routing configuration from the handler implementation

## Handler Functions in Axum

Axum handlers are asynchronous functions that take extractors as parameters and return types that can be converted into responses:

```rust showLineNumbers
async fn register_user(
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // Process the request
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    
    // Return status code and JSON response
    (StatusCode::CREATED, Json(user.details().clone()))
}
```

The function signature reveals several important concepts:
- `Json(payload)`: An extractor that parses the request body as JSON
- `(StatusCode, Json<UserDetails>)`: A tuple return type that combines a status code and JSON response

## Dependency Injection and Shared State

.NET has a built-in dependency injection system, but Rust takes a different approach. In Axum, you'll typically manage shared state using `Extension`:

```rust showLineNumbers
// Define your application state
#[derive(Default)]
struct AppState {
    users: Vec<User>,
}

// Create a shareable, thread-safe wrapper
type SharedState = Arc<RwLock<AppState>>;

// Configure your router with the shared state
let app = Router::new()
    .route("/users", post(register_user))
    .layer(Extension(SharedState::default()));

// Access the state in your handler
async fn register_user(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // Use the shared state
    state.write().unwrap().users.push(user.clone());
    // ...
}
```

This pattern allows multiple request handlers to safely share and modify application state across concurrent requests:

1. `Arc` (Atomic Reference Counting) provides thread-safe shared ownership of the state
2. `RwLock` (Read-Write Lock) ensures safe concurrent access to the mutable state
3. `Extension` injects the shared state into route handlers

## JSON Serialization in Web APIs

Axum integrates seamlessly with serde for JSON handling:

```rust showLineNumbers
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterUserRequest {
    email_address: String,
    password: String,
    name: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UserDetails {
    email_address: String,
    password: String,
    age: Option<i32>,
    name: String,
}
```

The `Json` extractor and response type handle the conversion between Rust types and JSON:

```rust showLineNumbers
// Extract JSON from request body
Json(payload): Json<RegisterUserRequest>

// Return JSON in response
Json(user.details().clone())
```

## Your Challenge

Now it's time to put what you've learned into practice! In this module's challenge, you'll build a simple web API that allows users to register and retrieve user information:

1. Add the Tokio and Axum dependencies to your project's Cargo.toml file
2. Create a main function with the tokio::main attribute
3. Implement the following API endpoints:
   - POST /users - Register a new user
   - GET /users/:email - Get a user by email address
4. Use shared state to store and retrieve users
5. Process JSON requests and return JSON responses with proper status codes
6. Test your API using a tool like Postman, curl, or your web browser

Your API should:
- Accept a JSON payload with email, name, and password fields
- Store the user in memory (no database required yet)
- Return a 201 Created status code for successful registration
- Return the user details (excluding password) in the response

The starter code is available in `src/module5/rust_app`, and you can check your solution against `src/module5/rust_app_final`.

Good luck, and remember that although the approach differs from ASP.NET Core, the concepts of routing, request handling, and state management translate well to Rust!