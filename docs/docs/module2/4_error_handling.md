---
sidebar_position: 4
---
# Error Handling

While not listed in the initial bullets, error handling differs significantly between the languages and is worth understanding.

In .NET, you're accustomed to exception handling with try/catch:

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