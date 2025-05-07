---
sidebar_position: 2
---

# Rust's Module System

Rust provides a module system that allows you to organize code into logical units and control visibility. Unlike namespaces in C#, Rust modules also control item visibility through access modifiers.

### Declaring Modules

In Rust, you can declare modules in two ways:

1. **In-file module declarations**:

```rust showLineNumbers
mod core {
    // Module contents
}
```

2. **Separate files**:

```rust showLineNumbers
// In main.rs
mod core; // Tells Rust to look for core.rs or core/mod.rs
```

The second approach is more common for larger applications and is what you'll use in this module.

:::info
When you declare a `mod core` the compiler is going to look for a file called `core.rs` **OR** `core/mod.rs`
:::

### Module Visibility

Rust has a powerful system of access modifiers. Think of these in a similar way to how you might think of encapsulation  in .NET (public, private, internal etc).

:::important
Everything is private to the current file as a default in Rust
:::

- No modifier: Private to the current module
- `pub`: Public, visible everywhere
- `pub(crate)`: Visible only within the current crate
- `pub(super)`: Visible to the parent module
- `pub(in path)`: Visible to a specific path

These modifiers help you enforce architectural boundaries.

