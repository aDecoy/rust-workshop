---
sidebar_position: 2
---

# Tokio

Tokio is a runtime for asynchronous programming in Rust. While .NET has built-in support for async/await, Rust's async functionality is provided by external crates. Tokio is the most popular async runtime and provides:

- A multi-threaded runtime for executing asynchronous code
- Utilities for asynchronous I/O operations
- Synchronization primitives for concurrent code
- Tools for working with time (delays, timeouts)

Tokio is essential for building network services because it allows your application to handle many connections concurrently without blocking.

### The #[tokio::main] Attribute

The `#[tokio::main]` attribute macro transforms your `main` function into one that initializes the Tokio runtime:

```rust showLineNumbers
#[tokio::main]
async fn main() {
    // Your asynchronous code here
}
```

Under the hood, this expands to code that creates a runtime and runs your async main function to completion.