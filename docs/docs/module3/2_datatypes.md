---
sidebar_position: 2
---

# Data Types

:::info

By convention, variables in Rust use snake case `_` instead of camel case

:::

### Basic Data Types

```rust showLineNumbers
// A string slice is a view into a string, or a reference to a string.
// It's fixed size that's known at compile time,
// It's does not require heap allocation
// Comparable with Span<String> in .NET
// Idiomatic Rust use snake case instead of camelCase
let str_example: &str = "Hello";
println!("{}", str_example);

// A string is a heap-allocated string. It's growable, and can be modified
let string_example: String = "Hello".to_string();
println!("{}", string_example);

let integer_example: i32 = 10;
println!("{}", str_example);

let float_example: f32 = 10.0;
println!("{}", float_example);

// Arrays are fixed size in Rust, and the size is declared at initialization
let array_example: [i32; 3] = [1, 2, 3];
println!("{}", array_example.len());

// Vecs are growable, heap-allocated arrays. They are the most commonly used collection in Rust.
// Vecs are similar to Lists in C#
// to add values to a Vec, you use the push method and the vec itself must be mutable
let vec_example: Vec<i32> = vec![1, 2, 3];
println!("{}", vec_example.len());

let bool_example: bool = true;
println!("{}", bool_example);
```

Another interesting thing about Rust is that `null` does not exist. Yep, you read that right. No more `Object reference not set to the instance of an object` errors.

But how do you model a variable that may or may not have a value. That's where the `Option` enum comes in:

## The Option Enum

In Rust, the Option enum is the way to represent a value that might be present or absent. Instead of using null references which can cause runtime errors, Rust forces you to explicitly handle both cases.

### Structure of Option
The Option type is defined as:

```rust showLineNumbers
enum Option<> {
    Some(T),  // Contains a value of type T
    None,     // Represents no value
}
```

Where T is a generic type parameter that can be any type.

### Using Option
Here's a simple example of using Option:

```rust showLineNumbers
let mut optional_string: Option<String> = None;

optional_string = Some("Hello".to_string());

```

This approach forces you to consider the case where a value might be absent, preventing many common bugs related to null references. The compiler won't let you use an `Option` as if it were definitely a T - you must handle both possibilities.

