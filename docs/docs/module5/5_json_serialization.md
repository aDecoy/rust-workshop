---
sidebar_position: 5
---

# JSON Serialization in Web APIs

Axum integrates seamlessly with `serde` for JSON handling:

```rust showLineNumbers
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterUserRequest {
    email_address: String,
    password: String,
    name: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UserDetails {
    email_address: String,
    password: String,
    age: Option<i32>,
    name: String,
}
```

The `Json` extractor and response type handle the conversion between Rust types and JSON:

```rust showLineNumbers
// Extract JSON from request body
Json(payload): Json<RegisterUserRequest>

// Return JSON in response
Json(user.details().clone())
```