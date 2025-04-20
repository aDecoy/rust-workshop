---
sidebar_position: 2
---

# Mutability

Every variable in Rust is immutable by default. All those variables you see above, yep they can never be changed.

To see this in action, go and run the `module3` code:

```sh
cd src/module_3/rust_app
cargo run
```

And oops! There's a problem:

**error[E0384]: cannot assign twice to immutable variable `integer_example`**

Let's see if you can go and fix it... Go on, try on your own.

If you delete the line assigning the `integer_example` then congrats 🎉 You're a Rust developer now.

So how would you go about declaring a variable that could be changed? That's easy, introducing the `mut` keyword.

```rs
// Shadowing
let mut str_example = "This string is now mutable";
str_example = "And can be edited";
println!("{}", str_example);
```

## Shadowing

```rs
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