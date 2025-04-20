---
sidebar_position: 1
---

# Intro

- Explain the SQLX crate and the benefits of using it 
- Initializing a new struct
- Getting the database URL as an environment variable
```rs
let db_url = &env::var("DATABASE_URL")
    .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;
```
- Explain `map_err` and the `?` syntax
- Writing SQL queries in Rust
- Creating the `PostgresDataAccess` instance as part of application startup
- Explain the SQLX CLI
- cargo sqlx migrate add

```sh
cd src/module9
docker compose up -d
export DATABASE_URL=postgresql://postgres:mysupersecretlocalpassword@localhost:5432/users
cd rust_app
cargo sqlx migrate run
cargo test
```