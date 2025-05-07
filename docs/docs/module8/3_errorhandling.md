---
sidebar_position: 3
---

# Error Handling

```rust showLineNumbers
impl PostgresUsers {
    pub async fn new(connection_string: String) -> Result<Self, ApplicationError> {
        let database_pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

        Ok(Self {
            db: database_pool,
        })
    }
}
```

This code introduces two important Rust error handling patterns:

### The `?` operator

This automatically propagates the error back up the stack. For this to work, the current function needs to return a `Result` and the error you are propagating needs to be the same as the `Error` of the current function. In the above example, the `new()` function uses a custom `ApplicationError` enum.

That means if you want to propagate an error back up the stack, the function call you add the `?` to also needs to return an `ApplicationError`.

The code sample above is trying to retrieve an environment variable, and the `env::var` function definitely doesn't return an `ApplicationError`. That's where `map_err` comes in.

### The `map_err` function`

The `map_err` function allows you to convert an error from one type to another. In the above example, you're taking the error returned by the `env::var` function, and creating a new `ApplicationError::DatabaseError` passing in the actual error as a string. Because you now have an `ApplicationError`, you can then propagate that back up the stack using the `?` syntax.

You might be thinking, where has that `ApplicationError` type come from though.

## Error Handling with `thiserror`

For structured error handling, you'll use the [`thiserror`](https://github.com/dtolnay/thiserror) crate to define application-specific errors:

```rust showLineNumbers
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("the provider password is incorrect")]
    IncorrectPassword,
    #[error("error interacting with database {0}")]
    DatabaseError(String),
    #[error("unexpected application error {0}")]
    ApplicationError(String),
}
```