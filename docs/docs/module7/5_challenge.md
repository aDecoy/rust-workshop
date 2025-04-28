---
sidebar_position: 5
---

# Your Challenge

Now it's time to apply what you've learned about traits, generics, and lifetimes in Rust! In this module's challenge, you'll:

1. Define a `DataAccess` trait with the following methods:
   - `async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError>;`
   - `async fn store(&self, user: User) -> Result<(), ApplicationError>;`

2. Create an `InMemoryDataAccess` implementation of this trait:
   - Use a `Mutex<Vec<User>>` to store users in memory
   - Implement the trait methods to find and store users

3. Refactor your application state to use generics with trait bounds:
   - Create a generic `AppState<TDataAccess>` type where TDataAccess has the bounds `DataAccess + Send + Sync`
   - Update your main function to instantiate the state with your implementation

4. Update your handler functions to work with the generic state:
   - Make the API handlers generic over the data access type
   - Use the data access trait methods instead of directly manipulating the user collection

:::note

Remember to think about whether the new data types you define should be in the `core`, `data_access` or `main` modules.

:::

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module7/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module7/rust_app). But try it on your own first, if you're finding it difficult that's good. It means you're learning.

By completing this challenge, you'll create a much more flexible application architecture that allows you to swap out data access implementations (for example, to use a database in the future) without changing your business logic or API handlers.

Good luck, and remember that while traits, generics, and lifetimes might seem complex at first, they provide powerful tools for building safe and flexible Rust applications!