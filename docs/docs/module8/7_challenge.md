---
sidebar_position: 7
---

# Challenge

Now it's time to apply what you've learned about database access with SQLx! In this module's challenge, you'll:

1. Set up a PostgreSQL database using Docker:
   - Use the provided docker-compose.yml file to run a PostgreSQL instance
   - Connect to the database using the correct connection string

   ```sh
   export DATABASE_URL=postgresql://postgres:mysupersecretlocalpassword@localhost:5432/users
   ```

2. Add SQLx to your project:
   - Update your Cargo.toml to include `sqlx` with the postgres and runtime-tokio features enabled
   - Add `thiserror` for better error handling

3. Create a database implementation of the DataAccess trait:
   - Create a PostgresUsers struct that connects to your database
   - Implement the trait methods to query and store users in PostgreSQL
   - Add proper error handling for database operations

4. Create and run a migration to set up your database schema:
   - Install the [SQLx CLI](https://crates.io/crates/sqlx-cli)
   - Create a migration to create a users table
   - Run the migration to apply your schema

   ```sh
   sqlx migrate add <MIGRATION_NAME>
   ```

5. Update your API handlers to work with the new implementation:
   - Initialize the PostgresUsers data access in your main function
   - Make sure all API endpoints correctly handle database errors

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module8/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module8/rust_app). But try it on your own first, if you're finding it difficult that's good. It means you're learning.

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