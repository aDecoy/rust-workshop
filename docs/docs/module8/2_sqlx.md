---
sidebar_position: 2
---

# Introduction to SQLx

SQLx is a Rust library that provides type-safe database interactions without an ORM. Key features include:

- **Compile-time SQL verification**: Checks your SQL queries at compile time
- **Native async support**: Fully asynchronous database operations
- **Type safety**: Maps database results to Rust types
- **No runtime reflection**: Zero-cost abstractions for database access
- **Support for multiple databases**: PostgreSQL, MySQL, SQLite, and more

For you as a .NET developer, SQLx is more like Dapper than Entity Framework, focusing on SQL queries rather than an ORM abstraction layer.

## Setting Up PostgreSQL

In this module, you'll use Docker to run a local PostgreSQL instance:

```yaml
# docker-compose.yml
services:
  postgres:
    image: postgres:17-alpine
    ports:
      - 5432:5432
    environment:
      - POSTGRES_PASSWORD=mysupersecretlocalpassword
      - POSTGRES_DB=users
```

You can start the database with:

```bash
docker compose up -d
```

## Adding SQLx to Your Project

First, add SQLx to your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio"] }
```

The features you select determine which database drivers and runtime support are included.

## Connecting to the Database

To connect to PostgreSQL, you'll need to:

1. Get the database URL from an environment variable
2. Create a connection pool
3. Handle connection errors appropriately

Examine how this is implemented:

```rust showLineNumbers
use std::env;
use sqlx::PgPool;
use crate::core::ApplicationError;

pub struct PostgresUsers {
    db: PgPool,
}

impl PostgresUsers {
    pub async fn new() -> Result<Self, ApplicationError> {
        // Get database URL from environment variable
        let db_url = &env::var("DATABASE_URL")
            .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

        // Create connection pool
        let database_pool = PgPool::connect(db_url)
            .await
            .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

        // Return new instance with connection pool
        Ok(Self {
            db: database_pool,
        })
    }
}
```