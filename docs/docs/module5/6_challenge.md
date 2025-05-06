---
sidebar_position: 5
---

# Challenge

Now it's time to put what you've learned into practice! In this module's challenge, you'll build a web API that allows users to register and retrieve user information:

1. Add the `tokio` and `axum` dependencies to your project's Cargo.toml file
2. Create a main function with the tokio::main attribute
3. Implement the following API endpoints:
   - `POST /users` - Register a new user
   - `GET /users/{email_address}` - Get a user by email address
4. Use shared state to store and retrieve users
5. Process JSON requests and return JSON responses with proper status codes
6. Test your API using a tool like Postman, curl, or your web browser

Your API should:
- Accept a JSON payload with email, name, and password fields
- Store the user in memory (no database required yet)
- Return a 201 Created status code for successful registration
- Return the user details (excluding password) in the response

:::info

Remember, you're still going to want to keep all the code you've already added for your `User` enum

:::

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module5/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module5/rust_app). Try it on your own first, if you're finding it difficult that's good. It means you're learning.

Good luck, and remember that although the approach differs from ASP.NET Core, the concepts of routing, request handling, and state management translate well to Rust!