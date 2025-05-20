---
sidebar_position: 3
---

# Dependency Injection

## Dependency Injection and Shared State

.NET has a built-in dependency injection system, but Rust takes a different approach. In Axum, you'll typically manage shared state using [`Extension`](https://docs.rs/axum/latest/axum/struct.Extension.html):

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
    .with_state(SharedState::default());

// Access the state in your handler
async fn register_user(
    State(state): State<SharedState>,
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

:::info

The `.clone()` function takes a complete clone of the value in memory. `let value = x.clone()` will make a copy of the variable `x`, that the variable `value` then owns.

Integers, bools and chars are `Copy` types, assignment or passing them around just copies the value, not the ownership. Same behaviour, you just don't need to be explicit about it like you would with `String`, `Vec`, or other complex types.

:::


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

Extensions can be used in Axum middleware to inject data into the request as part of some middleware that is running.

```rust showLineNumbers

```

### State (for Shared State)

`State` in Axum works in a similar way to `Extension`, it allows you to inject implementations into your handlers.

```rust showLineNumbers
#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        // When the /users route is hit, and it'a post request. 
        // Call the register_user function 
        .route("/users", post(register_user))
        .route("/users/{email_address}", get(get_user_details))
        .with_state(SharedState::default());
}

async fn register_user(
    State(state): State<SharedState>,
    // this argument tells axum to parse the request body
    // as JSON into a `RegisterUserRequest` type
    Json(payload): Json<RegisterUserRequest>,
){}

```

The big difference between `Extension` and `State` is that state is **type safe**, extension is not. In practice, that means if you try to use `State<T>` implementation in your handler at compile time you will get an error if you haven't registered a type that matches. `Extension` will not error at compile time, but will fail at runtime.