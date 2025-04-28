---
sidebar_position: 4
---

# Challenge

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

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module9/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module9/rust_app). But try it on your own first, if you're finding it difficult that's good. It means you're learning.

Good luck! Remember that comprehensive testing is a key part of building reliable Rust applications, and Rust's type system works with you to catch many potential issues at compile time.
