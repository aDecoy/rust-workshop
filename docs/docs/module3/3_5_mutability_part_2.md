---
sidebar_position: 3
---

# Mutability Part 2

If you delete the line assigning the `integer_example` then congrats 🎉 You're a Rust developer now.

So how would you go about declaring a variable that you could change? That's easy, introducing the `mut` keyword.

:::important
The `mut` keywords explicitally declares a variable as something you want to mutate later. If you declare a variable as mutable but never actually change it, the compiler will give you a warning and recommend you remove the `mut` keyword. Neat!
:::

```rust showLineNumbers
let mut str_example = "This string is now mutable";
str_example = "And can be edited";
println!("{}", str_example);
```

## Shadowing

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