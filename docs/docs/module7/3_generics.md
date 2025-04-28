---
sidebar_position: 3
---

# Generics

Generics in Rust serve a similar purpose to generics in C#: they allow you to write code that works with different types while maintaining type safety.

### Basic Generic Types

In your application, you can define a generic `AppState` type:

```rust showLineNumbers
pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess
}
```

This defines a struct that can work with any type `TDataAccess` that implements the `DataAccess` trait and is thread-safe (implements `Send` and `Sync`).

### Trait Bounds

The syntax `TDataAccess: DataAccess + Send + Sync` specifies trait bounds, similar to generic constraints in C#. It requires that the type parameter `TDataAccess` implements:

1. Your custom `DataAccess` trait
2. The `Send` trait (can be transferred between threads)
3. The `Sync` trait (can be shared between threads)

This is similar to a `where` clause in C#, but Rust's trait bounds can be more powerful.

### Generic Functions

You can also define generic functions with trait bounds:

```rust showLineNumbers
async fn register_user<TDataAccess: DataAccess + Send + Sync>(
    State(state): State<Arc<AppState<TDataAccess>>>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // Implementation...
}
```

This function works with any type that satisfies the trait bounds, providing flexibility while maintaining type safety.