---
sidebar_position: 1
---

# Memory Management

Ok, you got me. There are some pretty fundamental differences between Rust and .NET that you'll encounter as you make this transition. Understanding these differences is key to your success with Rust.

## 1. Memory Management

In .NET, you're used to the garbage collector handling memory for you. When you create objects, the runtime allocates memory, and when those objects are no longer referenced, the garbage collector eventually reclaims that memory.

In Rust, you'll encounter a completely different approach:
- **No Garbage Collection**: Rust doesn't use a garbage collector, which means no unpredictable pauses or runtime overhead.
- **Deterministic Cleanup**: Memory is freed at predictable points in your code—specifically, when variables go out of scope.
- **RAII (Resource Acquisition Is Initialization)**: Resources like file handles, network connections, and memory are tied to object lifetimes and automatically cleaned up when they're no longer needed.

This deterministic memory management gives you greater control and predictability. Your Rust programs will typically use less memory than equivalent .NET applications and have more consistent performance characteristics since there are no garbage collection pauses.

## 2. Borrow Checker

The borrow checker is often cited as Rust's most distinctive feature—and initially, its most challenging one for newcomers.

In .NET, you can freely create multiple references to the same object:

```csharp showLineNumbers
var myList = new List<int> { 1, 2, 3 };
var ref1 = myList;
var ref2 = myList;
// Both references can modify the list
ref1.Add(4);
ref2.Add(5);
```

Rust's borrow checker enforces strict rules about references:

- **Single Mutable Reference OR Multiple Immutable References**: You can have either one mutable reference to data or any number of immutable references, but never both simultaneously.
- **Compile-Time Verification**: These rules are enforced at compile time, preventing entire classes of bugs.
- **Error Messages**: When you violate these rules, the compiler gives you detailed messages explaining what went wrong and how to fix it.

This might seem restrictive at first, but it's this very restriction that enables Rust to guarantee thread safety and prevent data races without runtime checks. As you become familiar with the borrow checker, you'll find yourself thinking differently about how data flows through your program, leading to more robust designs.

## 3. Ownership

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

## 4. Lifetimes

Lifetimes are Rust's way of ensuring that references are always valid. They're how the compiler tracks how long references should be valid.

In .NET, you can create references that become invalid, leading to runtime errors:

```csharp showLineNumbers
string? GetDangerousReference() {
    var localString = "I'll be gone soon";
    return localString; // This actually works in C# because strings are special
    // But conceptually similar code with custom types could cause issues
}
```

In Rust, the compiler uses lifetimes to prevent these problems:
- **Lifetime Annotations**: Sometimes you need to explicitly tell the compiler how long references should live.
- **Preventing Dangling References**: The compiler ensures references never outlive the data they point to.
- **Lifetime Elision**: In common patterns, Rust can infer lifetimes automatically, so you don't always need to specify them.

Lifetimes are one of Rust's more advanced concepts, and you may not need to fully understand them immediately. The compiler will guide you when explicit lifetime annotations are needed, and over time, you'll develop an intuition for them.

## 5. Error Handling

While not listed in the initial bullets, error handling differs significantly between the languages and is worth understanding.

In .NET, you're accustomed to exception handling:

```csharp showLineNumbers
try {
    var file = File.OpenRead("missing.txt");
    // Process file
}
catch (FileNotFoundException ex) {
    Console.WriteLine("File not found");
}
```

Rust uses a different approach:
- **Result Type**: Functions that can fail return a `Result<T, E>` type instead of throwing exceptions.
- **Explicit Error Handling**: You must explicitly check for and handle errors; they can't be silently ignored.
- **? Operator**: The `?` operator provides a concise way to propagate errors up the call stack.
- **No Unchecked Exceptions**: All possible error paths are visible in function signatures.

:::important

You'll notice the use of the `.unwrap()` function in these early code samples. `.unwrap()` unwraps the Result type and returns the `OK` value. If the function has an error, then `.unwrap()` panics and crashes your application.

In most cases `.unwrap()` should be considered a code smell, but when you're learning it's ok to use it.

:::

This explicit approach to error handling makes code more reliable and encourages you to think about failure cases upfront. You'll find that Rust's error handling leads to more robust applications with fewer unexpected crashes.

## 6. Concurrency Model

Finally, concurrency works very differently between the two languages.

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

Remember that it's normal to feel frustrated at times—every Rust developer has experienced the same learning curve. But the payoff is worth it: you'll write code that's more reliable, more efficient, and free from entire classes of bugs that plague other languages.