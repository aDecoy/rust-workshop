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
    // .
}
```

This pattern allows multiple request handlers to safely share and modify application state across concurrent requests:

1. `Arc` (Atomic Reference Counting) provides thread-safe shared ownership of the state
2. `RwLock` (Read-Write Lock) ensures safe concurrent access to the mutable state
3. `Extension` injects the shared state into route handlers

## Understanding Rust Concurrency Primitives for .NET Developers

When working with Axum, you'll encounter several Rust primitives that manage concurrency and state. Here's how they compare to familiar .NET concepts:

### Arc (Atomic Reference Counting)

In .NET, the garbage collector handles memory management automatically. In Rust, `Arc` provides shared ownership across multiple parts of your program in a thread-safe way:

- **Purpose**: Enables multiple threads to share access to the same data
- **.NET Analogy**: Similar to a thread-safe immutable reference that multiple components can access
- **Key Difference**: In Rust, you explicitly choose `Arc` when you need shared ownership across threads
- **Usage**:

```rust showLineNumbers
// Create shared data
let shared_data = Arc::new(MyData::new());

// Clone the Arc to create another owner (only clones the pointer, not the data)
let worker_data = shared_data.clone();

// Pass to another thread
thread::spawn(move || {
    // Use worker_data in the new thread
});
```

### RwLock (Read-Write Lock)

`RwLock` is similar to .NET's `ReaderWriterLockSlim`:

- **Purpose**: Allows multiple concurrent readers OR a single writer, but never both simultaneously
- **.NET Equivalent**: `System.Threading.ReaderWriterLockSlim`
- **Key Difference**: Rust forces you to handle the lock result, preventing accidental lock leaks
- **Usage**:

```rust showLineNumbers
// Read access (shared among multiple readers)
let data = state.read().unwrap();
let value = data.some_field;
// Lock is automatically released when `data` goes out of scope

// Write access (exclusive)
let mut data = state.write().unwrap();
data.some_field = new_value;
// Lock is automatically released when `data` goes out of scope
```

### Extension (for Dependency Injection)

`Extension` in Axum provides dependency injection capabilities:

- **Purpose**: Makes shared resources available to HTTP handlers
- **.NET Equivalent**: ASP.NET Core's dependency injection system
- **Key Difference**: In Axum, you extract dependencies directly in handler parameters instead of constructor injection
- **Pattern**:

```rust showLineNumbers
// Register a dependency
let app = Router::new()
    .route("/endpoint", get(my_handler))
    .layer(Extension(my_shared_service));

// Access the dependency in a handler
async fn my_handler(
    Extension(service): Extension<MyService>
) -> impl IntoResponse {
    // Use the service
}
```