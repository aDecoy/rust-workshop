---
sidebar_position: 2
---

# Traits

In Rust, traits define shared behavior that types can implement. They're similar to interfaces in C#, but with some important differences.

## What is a Trait?

A trait is a collection of methods that a type must implement to satisfy the trait. For example a `DataAccess` interface in .NET might be defined as:

```csharp showLineNumbers
public interface IDataAccess {
    public Task Store(User user);
    public Task<User> WithEmailAddress(string emailAddress);
}
```

To implement a similar thing in Rust using traits, it would look like:

```rust showLineNumbers
pub trait DataAccess: Send + Sync {
    fn with_email_address(&self, email_address: &str) -> Option<User>;
    fn store(&self, user: User);
}
```

This trait defines two methods that any implementing type must provide. Like an interface in .NET, it specifies a contract without implementation details.

## Implementing Traits

As you learned in an earlier module, implementations for a given struct are defined in a seperate `impl {}` block. To implement a trait, you use the `impl Trait for Type` syntax:

```rust showLineNumbers
impl DataAccess {
    // Implementations specific to the DataAccess trait
}

// Implementations for the `InMemoryDataAccess` trait
impl DataAccess for InMemoryDataAccess {
    fn with_email_address(&self, email_address: &str) -> Option<User> {
        self.users.lock().unwrap().iter()
            .find(|u| u.email_address() == email_address)
            .cloned()
    }

    fn store(&self, user: User) {
        self.users.lock().unwrap().push(user);
    }
}
```

This is similar to implementing an interface in C#, but note that the implementation are separate from the type definition.

## Traits vs. Interfaces: Key Differences

1. **Orphan Rule**: In Rust, you can only implement a trait for a type if either the trait or the type is defined in your crate
2. **Default Implementations**: Traits can provide default implementations for methods
3. **Coherence**: A type can only have one implementation of a trait
4. **Static Dispatch**: Rust typically resolves trait methods at compile time (zero cost)

## Async Traits

When working with async functions in traits, you'll encounter a limitation: Rust doesn't *(currently, but it's in [progress](https://rust-lang.github.io/rust-project-goals/2025h1/async.html))* directly support async functions in traits yet. This is where the `async_trait` crate comes in:

```rust showLineNumbers
use async_trait::async_trait;

#[async_trait]
pub trait AsyncDataAccess {
    async fn with_email_address(&self, email_address: &str) -> Option<User>;
    async fn store(&self, user: User);
}
```

The `#[async_trait]` macro transforms the async methods into functions that return `Future` implementations, making them compatible with Rust's current trait system.

As well as adding `#[async_trait]` on the trait itself, you also need to add the macro to the implementation block as well:

```rust showLineNumbers
use async_trait::async_trait;

#[async_trait]
impl AsyncDataAcess for DataAccessStruct {
    async fn with_email_address(&self, email_address: &str) -> Option<User> {
        // Actual implementatoons
    }
    async fn store(&self, user: User)  {
        // Actual implementatoons
    }
}
```