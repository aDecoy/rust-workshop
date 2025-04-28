---
sidebar_position: 4
---

# Lifetimes

Lifetimes are a unique feature of Rust that helps prevent memory safety issues like dangling references.

## What are Lifetimes?

Lifetimes are annotations that help the Rust compiler ensure references remain valid. They describe how long references must be valid and help prevent using references after they've been freed.

### Basic Lifetime Syntax

Lifetime parameters are annotated with an apostrophe:

```rust showLineNumbers
fn example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

This function says that the returned reference will live at least as long as the shortest-lived of the input references.

## Why Lifetimes Matter

Without lifetimes, it would be possible to return references to data that has been freed, causing undefined behavior. Consider this invalid code:

```rust showLineNumbers
fn invalid_code() -> &str {
    let s = String::from("hello");
    &s  // INVALID: returns a reference to s, which is dropped at the end of the function
}
```

The compiler's lifetime system prevents such errors at compile time.

### Common Lifetime Patterns

1. **`'static`**: References that live for the entire program duration
2. **Elided lifetimes**: Simple cases where the compiler can infer lifetimes
3. **Lifetime bounds on generics**: Ensuring references in generic types are valid