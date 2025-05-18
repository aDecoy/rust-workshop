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

## Build a Web API with Axum

Examine how to build a basic API in Rust compared to .NET:

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

## Extractors

A handler function in an API is an async function that takes any number of `extractors` as arguments. You can think of extractors simply as a something that extracts *something* from the inbound request. For example, the `Json` extractor consumes the request body and deserilies it into a target type.

[Common extractors, taken from the Axum documentation](https://docs.rs/axum/latest/axum/extract/index.html), are:

```rust, showLineNumbers

// `Path` gives you the path parameters and deserializes them. See its docs for
// more details
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// `HeaderMap` gives you all the headers
async fn headers(headers: HeaderMap) {}

// `String` consumes the request body and ensures it is valid utf-8
async fn string(body: String) {}

// `Bytes` gives you the raw request body
async fn bytes(body: Bytes) {}

// We've already seen `Json` for parsing the request body as json
async fn json(Json(payload): Json<Value>) {}

// Parse the body using `application/x-www-form-urlencoded`
async fn accept_form(Form(sign_up): Form<SignUp>) {}

// `Request` gives you the whole request for maximum control
async fn request(request: Request) {}

```

Be aware, the [order the extractors](https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors) run in might be important. 

Extractors are a powerful part of the Axum ecosystem and it's worth taking some time to explore the Axum documentation in more detail.