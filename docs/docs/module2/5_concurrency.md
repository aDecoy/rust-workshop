---
sidebar_position: 5
---
# Concurrency Model

Finally, concurrency works differently between the two languages.

In .NET, you use various synchronization primitives to avoid race conditions:

```csharp showLineNumbers
private static readonly object _lock = new object();

public void UpdateSharedState() {
    lock (_lock) {
        // Safe to modify shared state here
    }
}
```

Rust's approach is to prevent data races at compile time:
- **Ownership and Borrowing Rules**: These extend to threads, preventing unsafe concurrent access.
- **Send and Sync Traits**: These traits control which types can be shared between threads.
- **Explicit Synchronization**: When you do need shared mutable state, you use types like `Mutex<T>` and `Arc<T>` that make thread safety explicit.

Rust's approach to concurrency eliminates many common bugs before your program ever runs. While there's a learning curve, you'll gain confidence writing concurrent code that "just works" without subtle race conditions.

## Learning to Embrace the Differences

As you work through this workshop, you'll find that these differences, while challenging at first, are what give Rust its unique strengths. Rather than fighting against them, learn to work with them. The Rust compiler will be your guide, pointing out potential issues and helping you write safer, more efficient code.

Remember that it's normal to feel frustrated at times—every Rust developer has experienced the same learning curve. The payoff is worth it: you'll write code that's more reliable, more efficient, and free from entire classes of bugs that plague other languages.