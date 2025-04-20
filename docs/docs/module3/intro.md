---
sidebar_position: 1
---

# Data Types

Let's get into starting to model our business domain. No low level systems programming here, let's build something real.

If you recall, functionality wise we need to build a service that allows users to:

- Register a new account
- Login
- Retrieve their account information
- Update their account details

Which means we need to model a `User`. `Users` are also going to need properties, you know things like `first_name`, `last_name`. So let's have a look at the different data types available in Rust.

> [!IMPORTANT]
> By convention, variables in Rust use `_` instead of `camelCase` or `snakeCase`

```rs
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