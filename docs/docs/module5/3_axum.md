---
sidebar_position: 3
---

# Axum Web Framework

Axum is a web framework built on top of Tokio. Created by the same team, it's designed to be:

- Modular and composable
- Type-safe and ergonomic
- Compatible with the broader Tokio ecosystem

For you as a .NET developer, Axum serves a similar role to ASP.NET Core, but with a more functional and composable approach.

### Key Components of Axum

1. **Routers**: Define the routes your API will handle
2. **Handlers**: Functions that process requests and return responses
3. **Extractors**: Types that extract data from requests
4. **Middleware**: Components that process requests before they reach handlers

## Building a Simple API with Axum

Let's examine how to build a basic API in Rust compared to .NET:

### Rust (with Axum):

```rust showLineNumbers
use axum::{
    routing::post,
    Router,
};

#[tokio::main]
async fn main() {
    // Create a new router
    let app = Router::new()
        .route("/users", post(register_user));

    // Start the server on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn register_user() -> &'static str {
    "User registered"
}
```

### .NET (with ASP.NET Core):

```csharp showLineNumbers
var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapPost("/users", () => "User registered");

app.Run("http://0.0.0.0:3000");
```

While both examples are concise, the approaches differ:

1. In Rust, routes are explicitly defined and associated with handler functions
2. In .NET, the minimal API approach uses method chaining with inline lambda expressions
3. Rust separates the routing configuration from the handler implementation

## Handler Functions in Axum

Axum handlers are asynchronous functions that take extractors as parameters and return types that can be converted into responses:

```rust showLineNumbers
async fn register_user(
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, Json<UserDetails>) {
    // Process the request
    let user = User::new(&payload.email_address, &payload.name, &payload.password);
    
    // Return status code and JSON response
    (StatusCode::CREATED, Json(user.details().clone()))
}
```

The function signature reveals several important concepts:
- `Json(payload)`: An extractor that parses the request body as JSON
- `(StatusCode, Json<UserDetails>)`: A tuple return type that combines a status code and JSON response