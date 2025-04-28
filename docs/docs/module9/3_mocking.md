---
sidebar_position: 1
---

# Mocking in Rust

Mocking in Rust typically leverages traits. By creating trait implementations specifically for testing:

```rust showLineNumbers
// Define a trait for the behavior we want to mock
#[async_trait::async_trait]
pub trait DataAccess {
    async fn get_user(&self, id: &str) -> Result<User, Error>;
    async fn save_user(&self, user: &User) -> Result<(), Error>;
}

// Create a mock implementation for testing
struct MockDataAccess {
    // You can store expected results or track calls
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl DataAccess for MockDataAccess {
    async fn get_user(&self, id: &str) -> Result<User, Error> {
        match self.users.get(id) {
            Some(user) => Ok(user.clone()),
            None => Err(Error::NotFound),
        }
    }
    
    async fn save_user(&self, user: &User) -> Result<(), Error> {
        // For testing, just return Ok
        Ok(())
    }
}

// Test using the mock
#[tokio::test]
async fn test_user_service() {
    // Setup mock
    let mut mock_data = MockDataAccess { users: HashMap::new() };
    mock_data.users.insert("123".to_string(), User::new("123", "Test User"));
    
    // Create service with mock
    let service = UserService::new(mock_data);
    
    // Test service
    let user = service.get_user("123").await.unwrap();
    assert_eq!(user.name, "Test User");
}
```

This approach:
- Uses Rust's traits for abstraction
- Allows easy substitution of real implementations with test doubles
- Maintains type safety throughout

## Property-Based Testing

Property-based testing is a powerful technique where you define properties your code should satisfy, and the testing framework generates random inputs to verify these properties.

Using the `proptest` crate:

```rust showLineNumbers
use proptest::prelude::*;

proptest! {
    #[test]
    fn addition_is_commutative(a in 0..100i32, b in 0..100i32) {
        assert_eq!(add(a, b), add(b, a));
    }
    
    #[test]
    fn addition_is_associative(a in 0..100i32, b in 0..100i32, c in 0..100i32) {
        assert_eq!(add(add(a, b), c), add(a, add(b, c)));
    }
}
```

Property-based testing is particularly useful for:
- Complex algorithms where edge cases are hard to identify
- Functions where multiple properties can be verified
- Finding unexpected bugs with unusual inputs

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

## Your Challenge

Now it's time to apply what you've learned about testing in Rust! In this module's challenge, you'll:

1. Write unit tests for core functionality:
   - Add tests for the User model to verify email validation works correctly
   - Test password hashing and verification functionality
   - Ensure that the business logic in your core domain behaves as expected

2. Create an in-memory mock for DataAccess:
   - Implement a test-specific version of your DataAccess trait
   - Make it store data in memory for testing purposes
   - Use it to test your API handlers without needing a real database

3. Write integration tests for API endpoints:
   - Create tests in the `tests/` directory that verify your API endpoints
   - Test successful registration, login, and user retrieval
   - Test error cases like duplicate registration and invalid login

4. Add property-based tests:
   - Use the proptest crate to test properties of your password hashing
   - Verify that different inputs always produce different hashes
   - Test that verification always works for the correct password

5. Ensure all tests are passing:
   - Run `cargo test` and fix any failing tests
   - Make sure your test coverage is comprehensive

The starter code is available in `src/module9/rust_app`, and you can check your solution against `src/module9/rust_app_final`.

Good luck! Remember that comprehensive testing is a key part of building reliable Rust applications, and Rust's type system works with you to catch many potential issues at compile time.
