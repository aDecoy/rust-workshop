---
sidebar_position: 4
---

# SQL Queries

SQLx provides several ways to execute queries. The most powerful is the `query!` macro, which validates your SQL at compile time:

```rust showLineNumbers
async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError> {
    // Create the query and pass in parameters
    let email = sqlx::query!(
        r#"
        SELECT email_address, name, password
        FROM users
        WHERE email_address = $1
        "#,
        email_address,
    )
        // Query the database, returning an Option if no row is found
        .fetch_optional(&self.db)
        .await;
    
    // Check to see if the Option is Some or None
    match email {
        // If it is, parse the record
        Ok(record) => match record {
            Some(data) => {
                // Create a new User instance from the data
                let user = User::from(&data.email_address, &data.name, &data.password);
                Ok(user)
            },
            None => Err(ApplicationError::UserDoesNotExist)
        },
        Err(_) => Err(ApplicationError::UserDoesNotExist)
    }
}
```

Key points about this code:
- The `query!` macro generates type-safe code based on your SQL
- Parameters are passed using PostgreSQL's `$1`, `$2` syntax
- `fetch_optional` returns an `Option` that you can match on
- The record's fields are type-checked based on your database schema

## Implementing the DataAccess Trait

To maintain the abstraction from the previous module, you'll implement the `DataAccess` trait for your `PostgresUsers` struct:

```rust showLineNumbers
#[async_trait::async_trait]
impl DataAccess for PostgresUsers {
    async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError> {
        // SQL query implementation...
    }

    async fn store(&self, user: User) -> Result<(), ApplicationError> {
        let _rec = sqlx::query!(
            r#"
            INSERT INTO users (email_address, name, password)
            VALUES ($1, $2, $3)
            "#,
            user.email_address(),
            user.name(),
            user.password()
        )
            .fetch_one(&self.db)
            .await;

        Ok(())
    }
}
```

Note the `#[async_trait::async_trait]` attribute is required because Rust doesn't natively support async functions in traits yet.

## Database Migrations with SQLx CLI

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

## Your Challenge

Now it's time to apply what you've learned about database access with SQLx! In this module's challenge, you'll:

1. Set up a PostgreSQL database using Docker:
   - Use the provided docker-compose.yml file to run a PostgreSQL instance
   - Connect to the database using the correct connection string

2. Add SQLx to your project:
   - Update your Cargo.toml to include sqlx with the postgres and runtime-tokio features
   - Add thiserror for better error handling

3. Create a database implementation of the DataAccess trait:
   - Create a PostgresUsers struct that connects to your database
   - Implement the trait methods to query and store users in PostgreSQL
   - Add proper error handling for database operations

4. Create and run a migration to set up your database schema:
   - Install the SQLx CLI
   - Create a migration to create a users table
   - Run the migration to apply your schema

5. Update your API handlers to work with the new implementation:
   - Initialize the PostgresUsers data access in your main function
   - Make sure all API endpoints correctly handle database errors

The starter code is available in `src/module8/rust_app`, and you can check your solution against `src/module8/rust_app_final`.

To test your implementation, follow these steps:
```bash
# Start the PostgreSQL database
docker compose up -d

# Set the database URL
export DATABASE_URL=postgresql://postgres:mysupersecretlocalpassword@localhost:5432/users

# Run migrations and start the application
cargo sqlx migrate run
cargo run
```

Good luck, and remember that working with databases in Rust gives you the power of compile-time SQL checking, helping you catch errors before your code even runs!