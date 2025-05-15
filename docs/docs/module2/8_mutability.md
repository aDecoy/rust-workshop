---
sidebar_position: 8
---

# Mutability

:::important

Every variable in Rust is immutable by default.

:::

To see this in action, go and run the `module3` code:

```rust
let integer_example = 99;
integer_example = 12;
```

And oops! There's a problem:

**error[E0384]: cannot assign twice to immutable variable `integer_example`**

So how would you go about declaring a variable that you could change? That's easy, introducing the `mut` keyword.

:::important
The `mut` keywords explicitally declares a variable as something you want to mutate later. If you declare a variable as mutable but never actually change it, the compiler will give you a warning and recommend you remove the `mut` keyword. Neat!
:::

```rust showLineNumbers
let mut str_example = "This string is now mutable";
str_example = "And can be edited";
println!("{}", str_example);
```