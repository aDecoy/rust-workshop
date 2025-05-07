---
sidebar_position: 5
---

# Database Migrations

SQLx provides a CLI tool for managing database migrations. To use it:

1. Install the SQLx CLI:
   ```bash
   cargo install sqlx-cli
   ```

2. Create a migration:
   ```bash
   cargo sqlx migrate add create_users_table
   ```

3. Run migrations:
   ```bash
   cargo sqlx migrate run
   ```

A typical migration file might look like:

```sql
-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email_address TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    password TEXT NOT NULL
);
```

## Initializing the Database in Your Application

With all these pieces in place, you can initialize your database in the main function:

```rust showLineNumbers
#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Initialize database connection
    let postgres_data_access = PostgresUsers::new().await?;
    
    let shared_state = Arc::new(AppState{
        data_access: postgres_data_access
    });
    
    // Configure API routes
    let app = Router::new()
        .route("/users", post(register_user))
        .route("/login", post(login))
        .route("/users/{email_address}", get(get_user_details))
        .with_state(shared_state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;
    
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;
    
    Ok(())
}
```

Notice that the main function now returns a `Result` type, allowing you to use the `?` operator for clean error handling throughout.