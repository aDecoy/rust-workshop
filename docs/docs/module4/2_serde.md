---
sidebar_position: 2
---

# What is Serde?

Serde (short for "serialize" and "deserialize") is a framework for serializing and deserializing Rust data structures efficiently and generically. Unlike .NET's built-in JSON serialization, Serde is not specific to any particular data format - it's designed to be format-agnostic.

In Rust, you'll typically use two crates together:
- `serde`: The core serialization/deserialization framework
- `serde_json`: The JSON-specific implementation of serde

## Adding Serde to Your Project

First, you need to add the required dependencies to your `Cargo.toml` file:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

The `derive` feature is particularly important as it allows you to use procedural macros to automatically implement serialization for your types. 

What's a procedural macro you ask? Or even more fundamentally, what's a macro?
