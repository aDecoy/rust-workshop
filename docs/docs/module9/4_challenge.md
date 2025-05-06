---
sidebar_position: 4
---

# Challenge

Now it's time to apply what you've learned about testing in Rust! In this module's challenge, you'll:

1. Write unit tests for core functionality:
   - Add tests for the User model to verify email validation works correctly
   - Test password hashing and verification functionality
   - Ensure that the business logic in your core domain behaves as expected

2. Create a mock implementation of the DataAccess trait:
   - Implement a test-specific version of your DataAccess trait
   - Make it store data in memory for testing purposes
   - Use it to test your API handlers without needing a real database

3. Write integration tests for API endpoints:
   - Create a new directory and for running integration tests `mkdir integration-tests && cd integration-tests && cargo init`
   - Test successful registration and login

:::info

You'll need a couple of additional crates for writing integration tests. The dependencies in your `Cargo.toml` should be:

```toml
[dev-dependencies]
tokio = { version = "1.38", features = ["macros", "rt-multi-thread"] }
serde_json = "1.0"
reqwest = { version = "0.12", default-features = false, features = [
  "rustls-tls",
  "http2",
] }

[dependencies.uuid]
version = "1.16.0"
# Lets you generate random UUIDs
features = [
    "v4",
]
```

:::

4. Ensure all tests are passing:
   - Run `cargo test` and fix any failing tests

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module9/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module9/rust_app). Try it on your own first, if you're finding it difficult that's good. It means you're learning.

Good luck! Remember that comprehensive testing is a key part of building reliable Rust applications, and Rust's type system works with you to catch many potential issues at compile time.
