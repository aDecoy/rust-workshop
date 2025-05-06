---
sidebar_position: 3
---

# Generics

Generics in Rust serve a similar purpose to generics in C#: they allow you to write code that works with different types while maintaining type safety.

## Monomorphization

For this to work, Rust needs a way to figure out the concrete types of all the generics before the code can execute. Different languages handle this problem differently. Rust takes a different approach: it monomorphizes all generic types. This means that compiler stamps out a different copy of the code of a generic function for each concrete type needed. For example, if I use a Vec<u64> and a Vec<String> in my code, then the generated binary will have two copies of the generated code for Vec: one for Vec<u64> and another for Vec<String>. The result is fast programs, but it comes at the cost of compile time (creating all those copies can take a while) and binary size (all those copies might take a lot of space).

## Basic Generic Types

In your application, you can define a generic `AppState` type:

```rust showLineNumbers
pub struct AppState<TDataAccess: DataAccess + Send + Sync> {
    pub data_access: TDataAccess
}
```

This defines a struct that can work with any type `TDataAccess` that implements the `DataAccess` trait and is thread-safe (implements `Send` and `Sync`).

## Trait Bounds

The syntax `TDataAccess: DataAccess + Send + Sync` specifies trait bounds, similar to generic constraints in C#. It requires that the type parameter `TDataAccess` implements:

1. Your custom `DataAccess` trait
2. The `Send` trait (can be transferred between threads)
3. The `Sync` trait (can be shared between threads)

This is similar to a `where` clause in C#, but Rust's trait bounds can be more powerful.

## Generic Functions

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