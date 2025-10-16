# Modul 5: Bygge et web API

# Challenge

Now it's time to put what you've learned into practice! In this module's challenge, you'll build a web API that allows users to register and retrieve user information:

1. Add the axum dependency to your project's Cargo.toml file
2. Create a main function with the tokio::main attribute
3. Implement the following API endpoints:

   + POST /users - Register a new user
   + GET /users/{email_address} - Get a user by email address
4. Use shared state to store and retrieve users
5. Process JSON requests and return JSON responses with proper status codes
6. Test your API using a tool like Postman, curl, or your web browser
7. Your API should:

   + Accept a JSON payload with email, name, and password fields
   + Store the user in memory (no database required yet)
   + Return a 201 Created status code for successful registration
   + Return the user details (excluding password) in the response

Good luck, and remember that working with JSON in Rust gives you the benefits of strong type checking while maintaining high performance!

# Hint og teori

* Se på module5 teori https://rustfor.net/docs/category/your-first-web-api
* Tokio doc https://tokio.rs/tokio/tutorial/hello-tokio
