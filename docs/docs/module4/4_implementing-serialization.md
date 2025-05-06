---
sidebar_position: 4
---

# Implementing Serialization

To make your types serializable, you add the `Serialize` and `Deserialize` derive macros:

```rust showLineNumbers
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserDetails {
    email_address: String,
    age: Option<i32>,
    name: String,
}

#[derive(Serialize, Deserialize)]
enum User {
    Standard{user_details: UserDetails},
    Premium{user_details: UserDetails, is_premium: bool}
}
```

:::info

If you're familiar with [source generated serialization](https://learn.microsoft.com/en-us/dotnet/standard/serialization/system-text-json/source-generation) in .NET, Rust (de)serialization works in the same way. The code required to manipulate JSON is generated at **compile time**, not relying on things like reflection at runtime.

:::

These derive macros automatically implement the necessary traits, making your types serializable and deserializable.

## Serializing Data to JSON

Now that your types are set up, you can convert them to JSON strings:

```rust showLineNumbers
let user = User::new("james@eastham.com", "James");

// Convert to a pretty-printed JSON string
let json_string = serde_json::to_string_pretty(&user).unwrap();
println!("{}", json_string);
```

The `to_string_pretty` function converts your Rust object to a formatted JSON string with indentation. For more compact output, you can use `to_string` instead.

## Deserializing JSON to Rust Types

You can also convert JSON data back into Rust types:

```rust showLineNumbers
let user_json = r#"
    "email_address": "james@eastham.com",
    "name": "James",
}"#;

let user_details: UserDetails = serde_json::from_str(user_json).unwrap();
```

The `from_str` function parses a JSON string and converts it to a Rust type. The `unwrap()` call will panic if the JSON is invalid or doesn't match the expected structure - in production code, you'd want to handle errors more gracefully.

## Customizing JSON Serialization

Serde provides numerous attributes to customize how your types are represented in JSON:

### Field Renaming

```rust showLineNumbers
#[derive(Serialize, Deserialize)]
struct UserDetails {
    #[serde(rename = "email")]
    email_address: String,
    age: Option<i32>,
    name: String,
}
```

This will serialize `email_address` as `"email"` in the JSON output.

### Case Conversion

```rust showLineNumbers
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserDetails {
    email_address: String,
    age: Option<i32>,
    name: String,
}
```

This will convert all field names to camelCase in JSON (e.g., `emailAddress` instead of `email_address`).

### Skipping Fields

```rust showLineNumbers
#[derive(Serialize, Deserialize)]
struct UserDetails {
    email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<i32>,
    name: String,
}
```

This will omit the `age` field from JSON output if it's `None`.