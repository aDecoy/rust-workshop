---
sidebar_position: 1
---

# Testing in Rust

In this module, you'll learn how to implement effective testing practices in Rust. For you as a .NET developer, you'll find that while many testing concepts are similar to what you're used to, Rust's unique features enable some powerful testing approaches.

## Learning Objectives

By the end of this module, you will:
- Understand Rust's built-in testing framework
- Write unit tests and integration tests in Rust
- Use test fixtures and mocks in Rust
- Implement property-based testing with proptest
- Practice test-driven development in Rust

## Rust's Testing Philosophy

Rust has a built-in testing framework that's simple yet powerful. The testing philosophy is:

1. **Tests live close to the code they test**: Unit tests often live in the same file as the implementation
2. **No separate test framework needed**: Tests run with a simple `cargo test` command
3. **First-class support in the language**: Testing is a core feature of Rust, not an afterthought
4. **Compile-time guarantees reduce need for some tests**: Many bugs caught by tests in other languages are caught by the compiler in Rust

## Unit Testing in Rust

Unit tests in Rust typically live in the same file as the code they test, in a special module annotated with `#[cfg(test)]`. This attribute ensures the test code is only compiled when running tests.

Here's an example:

```rust showLineNumbers
// Implementation code
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Test module
#[cfg(test)]
mod tests {
    // Import parent scope
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
    
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }
}
```

Key points:
- `#[cfg(test)]` ensures test code is only included when testing
- `use super::*` imports all items from the parent module
- `#[test]` marks functions as test cases
- `assert_eq!`, `assert!`, and `assert_ne!` macros help with assertions

## Integration Testing

Integration tests live in a separate directory called `tests` at the root of your project. Each file in this directory is compiled as a separate crate.

```
my_project/
├── src/
│   └── lib.rs
└── tests/
    ├── api_tests.rs
    └── data_tests.rs
```

An integration test might look like:

```rust showLineNumbers
// tests/api_tests.rs
use my_project::api;

#[test]
fn test_api_endpoint_returns_expected_data() {
    let result = api::get_data();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 10);
}
```

Integration tests:
- Test your code's public API
- Verify that components work together correctly
- Run with the same `cargo test` command
- Access only public items from your crate

## Testing Async Code

To test async functions, you need to use a runtime like tokio:

```rust showLineNumbers
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_async_function() {
        let result = fetch_data().await;
        assert!(result.is_ok());
    }
}
```

The `#[tokio::test]` attribute sets up the tokio runtime for your async test.

## Test Fixtures

For tests that need similar setup and teardown, you can use Rust's Drop trait:

```rust showLineNumbers
struct TestFixture {
    // Test data and state...
    db_connection: PgPool,
}

impl TestFixture {
    async fn new() -> Self {
        // Setup code...
        let db_connection = PgPool::connect("test_db_url").await.unwrap();
        // Run migrations, seed data...
        
        Self { db_connection }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // Cleanup code...
        // This runs when the fixture goes out of scope
    }
}

#[tokio::test]
async fn test_with_fixture() {
    let fixture = TestFixture::new().await;
    
    // Test using fixture...
    let result = get_user(&fixture.db_connection, "test@example.com").await;
    assert!(result.is_ok());
    
    // Fixture is automatically cleaned up when it goes out of scope
}
```

## Mocking in Rust

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
