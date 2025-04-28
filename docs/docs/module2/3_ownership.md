---
sidebar_position: 3
---

# Ownership

Ownership is Rust's most fundamental concept and the foundation of its memory safety guarantees.

In .NET, multiple parts of your code can "own" an object simultaneously. Consider this C# example:

```csharp showLineNumbers
var data = new List<int> { 1, 2, 3 };
ProcessList(data);
// We can still use data here
data.Add(4);

void ProcessList(List<int> list) {
    // The method temporarily "borrows" the list
    list.Add(10);
}
```

In Rust, every value has exactly one owner:
- **Transfer of Ownership**: When you pass a value to a function or assign it to another variable, ownership transfers unless you explicitly borrow it (more on that later).
- **Move Semantics**: Assigning a value to another variable moves it, making the original variable invalid.
- **Copy Types**: Simple types like integers implement the `Copy` trait and are automatically copied instead of moved.

:::info

Don't worry, you'll learn more about `traits` in a later module. For the moment, think of them like interfaces

:::

This system may seem unusual at first, but it's what allows Rust to determine exactly when memory can be freed without relying on garbage collection. You'll learn patterns like borrowing and cloning that give you flexibility while maintaining memory safety guarantees.