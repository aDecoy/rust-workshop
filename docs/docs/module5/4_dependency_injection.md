---
sidebar_position: 3
---

# Dependency Injection

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