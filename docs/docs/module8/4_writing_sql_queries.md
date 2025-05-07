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
        // SQL query implementation.
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