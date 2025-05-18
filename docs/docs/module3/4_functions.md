---
sidebar_position: 4
---

# Functions

Functions in Rust are similar to methods in C#, but with some important differences in syntax and behavior.

## Basic Function Syntax

In Rust, functions are defined using the `fn` keyword:

```rust showLineNumbers
fn say_hello() {
    println!("Hello, world!");
}

// Call the function
say_hello();
```

## Parameters and Return Values

Rust functions are explicitly typed. You must declare the types of all parameters and return values:

```rust showLineNumbers
fn add(x: i32, y: i32) -> i32 {
    x + y  // Note: no semicolon here means this value is returned
}

let result = add(5, 3);
println!("The sum is: {}", result);
```

:::important
In Rust, the final expression in a function block is implicitly returned if it doesn't end with a semicolon. You can also use the `return` keyword for early returns.
:::

## The Unit Type

If a function doesn't return a value, it implicitly returns the unit type `()`, which is similar to `void` in C#:

```rust showLineNumbers
fn do_something() {
    println!("This function returns ()");
}

// This is equivalent to:
fn do_something_explicit() -> () {
    println!("This function explicitly returns ()");
}
```

## Error Handling in Functions

As you saw in the last module, unlike C# methods that can throw exceptions, Rust functions typically return a `Result<T, E>` enum to indicate success or failure:

```rust showLineNumbers
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("10 / 2 = {}", result),
    Err(e) => println!("Error: {}", e),
}

match divide(10.0, 0.0) {
    Ok(result) => println!("10 / 0 = {}", result),
    Err(e) => println!("Error: {}", e),  // Prints "Error: Cannot divide by zero"
}
```

This pattern forces callers to explicitly handle potential errors, making Rust code more robust and explicit about error conditions.

