---
sidebar_position: 1
---

# Mocking in Rust

Mocking in Rust typically leverages traits. By creating trait implementations specifically for testing. You can implement these mock inside your test module:

```rust showLineNumbers
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use crate::core::{ApplicationError, User};
    use std::sync::Arc;

    // Create a mock implementation for testing
    struct MockDataAccess {
        // You can store expected results or track calls
        users: HashMap<String, User>,
    }

    impl MockDataAccess {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }
    }

    #[async_trait::async_trait]
    impl DataAccess for MockDataAccess {
        async fn with_email_address(&self, email_address: &str) -> std::result::Result<User, ApplicationError> {
            if let Some(user) = self.users.get(email_address) {
                Ok(user.clone())
            } else {
                Err(ApplicationError::UserDoesNotExist)
            }
        }

        async fn store(&self, user: User) -> std::result::Result<(), ApplicationError> {
            // Simulate storing the user
            Ok(())
        }
    }
}
```

### The Mockall crate

Alternatively, you can use the [`mockall`](https://docs.rs/mockall/latest/mockall/) crate, which provides similar functionality to the [`moq`](https://github.com/devlooped/moq) library in .NET.

With mockall, you add the `#[automock]` macro to your trait, and that will generate a mock implementation of the trait that you can use in your tests.


```rust showLineNumbers
use mockall::{automock, mock};

#[async_trait::async_trait]
#[automock]
pub trait DataAccess {
    async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError>;
    async fn store(&self, user: User) -> Result<(), ApplicationError>;
}
```

You can also dynamically include the `#[automock] macro only if compiling for a test run, and excluding the mock code for a release build.

```rust showLineNumbers
// Only include the following line of code if a test is running
#[cfg(any(test, feature = "mocks"))]
use mockall::{automock, predicate::*};

#[async_trait::async_trait]
// Add the `automock` macro if a test run
#[cfg_attr(any(test, feature = "mocks"), automock)]
pub trait DataAccess {
    async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError>;
    async fn store(&self, user: User) -> Result<(), ApplicationError>;
}
```

Alternatively, you can use the `mock!` macro to generate a mock implementation for a trait outside of the current crate or module. This is useful for your case if you want to test the code in `main.rs`, but using a mock of the `DataAccess` trait which is defined in the `core` module. It also means you can keep the code in your core library clean from additional macros.

```rust showLineNumbers
mock! {
    DataAccess{}
    #[async_trait::async_trait]
    impl DataAccess for DataAccess {
        async fn with_email_address(&self, email_address: &str) -> std::result::Result<User, ApplicationError>;
        async fn store(&self, user: User) -> std::result::Result<(), ApplicationError>;
    }
}
```

Once you've defined your mock, you can then configure how it functions as part of your test run. The name of the generated struct will always be prefixed with the word `Mock`. For example, if your trait was called `DataAccess` then the auto-generated implementation would be called `MockDataAccess` and you would initialize it using the `::new()` function:

```rust showLineNumbers
// Initialize the mock implementation
let mut mock_data_access = MockDataAccess::new();
    mock_data_access
        // Configure the `store()` function
        .expect_store()
        // Expect the function to be passed a user struct, with the email address set to test@test.com
        .withf(|user| {
            user.email_address() == "test@test.com".to_string()
        })
        // Define the return value
        .return_once(move |_| Ok(()));
```

## Test-Driven Development in Rust

Test-Driven Development (TDD) works well in Rust:

1. Write a failing test that defines the expected behavior
2. Implement the minimum code needed to pass the test
3. Refactor while keeping the tests passing

The fast compile-test cycle in Rust makes TDD efficient, and the type system helps guide your implementation.

## Running Tests

Run your tests with:

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name

# Run tests with output (don't capture stdout)
cargo test -- --nocapture

# Run tests in release mode (optimized)
cargo test --release
```
