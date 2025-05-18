---
sidebar_position: 3
---

# Shadowing

When you're working with Rust, you'll often encounter a concept called shadowing. Here's how it works:

```rust showLineNumbers
// Shadowing
let mut str_example = "This string is now mutable";
str_example = "And can be edited";
println!("{}", str_example);
// PRINTS: And can be edited

// Shadowing also works inside a code block
{
    let str_example = "This is a new value";
    println!("{}", str_example);
    // PRINTS: This is a new value
}

// The value of str_example here is still the same as before the code block
println!("{}", str_example);
// PRINTS: And can be edited
```

With shadowing, you can declare a new variable with the same name as a previous variable. This is different from mutability - you're creating a completely new variable that happens to have the same name. This is particularly useful when you need to change the type of a variable, while keeping the same name.