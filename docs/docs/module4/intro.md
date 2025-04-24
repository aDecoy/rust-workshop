---
sidebar_position: 1
---

# Module 4: Working with JSON in Rust

In this module, you'll learn how to work with JSON data in Rust using the powerful `serde` ecosystem. For you as a .NET developer, this is similar to using `System.Text.Json` or `Newtonsoft.Json` to serialize and deserialize objects, but with the added safety and performance that Rust provides.

## Learning Objectives

By the end of this module, you will:
- Understand what the `serde` ecosystem is and how it works
- Learn about Rust macros and procedural macros
- Add JSON serialization and deserialization to your Rust types
- Customize how your data is represented in JSON

## What is Serde?

Serde (short for "serialize" and "deserialize") is a framework for serializing and deserializing Rust data structures efficiently and generically. Unlike .NET's built-in JSON serialization, Serde is not specific to any particular data format - it's designed to be format-agnostic.

In Rust, you'll typically use two crates together:
- `serde`: The core serialization/deserialization framework
- `serde_json`: The JSON-specific implementation of serde

## Step 1: Adding Serde to Your Project

First, you need to add the required dependencies to your `Cargo.toml` file:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

The `derive` feature is particularly important as it allows you to use procedural macros to automatically implement serialization for your types.

## Step 2: Understanding Rust Macros

Before diving deeper, let's understand what macros are in Rust:

### Declarative Macros

Declarative macros (also called "macros by example" or "macro_rules! macros") allow you to write code that resembles a match expression but operates on Rust code at compile time. A simple example is the `println!` macro.

### Procedural Macros

Procedural macros act more like functions (they take code as input, manipulate it, and produce code as output). The `derive` macros you'll use with serde are a type of procedural macro.

Serde's derive macros automatically implement the `Serialize` and `Deserialize` traits for your types, saving you from writing boilerplate code.

## Step 3: Implementing Serialization for Your Types

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

These derive macros automatically implement the necessary traits, making your types serializable and deserializable.

## Step 4: Serializing Data to JSON

Now that your types are set up, you can convert them to JSON strings:

```rust showLineNumbers
let user = User::new("james@eastham.com", "James");

// Convert to a pretty-printed JSON string
let json_string = serde_json::to_string_pretty(&user).unwrap();
println!("{}", json_string);
```

The `to_string_pretty` function converts your Rust object to a formatted JSON string with indentation. For more compact output, you can use `to_string` instead.

## Step 5: Deserializing JSON to Rust Types

You can also convert JSON data back into Rust types:

```rust showLineNumbers
let user_json = r#"
    "email_address": "james@eastham.com",
    "name": "James",
}"#;

let user_details: UserDetails = serde_json::from_str(user_json).unwrap();
```

The `from_str` function parses a JSON string and converts it to a Rust type. The `unwrap()` call will panic if the JSON is invalid or doesn't match the expected structure - in production code, you'd want to handle errors more gracefully.

## Step 6: Customizing JSON Serialization

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

## Your Challenge

Now it's time to put what you've learned into practice! In this module's challenge, you'll:

1. Add the serde and serde_json dependencies to your project's Cargo.toml file
2. Apply the `Serialize` and `Deserialize` derive macros to your User and UserDetails types
3. Modify your main function to:
   - Create a user and serialize it to JSON
   - Print the JSON to the console
   - Parse a JSON string back into a User struct
4. Experiment with customizing the JSON output by adding at least one serde attribute (like rename_all)
5. Ensure you can successfully compile and run your application to confirm the JSON serialization works

The starter code is available in `src/module4/rust_app`, and you can check your solution against `src/module4/rust_app_final`.

Good luck, and remember that working with JSON in Rust gives you the benefits of strong type checking while maintaining high performance!