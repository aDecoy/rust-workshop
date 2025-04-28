---
sidebar_position: 2
---

# Borrowing

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