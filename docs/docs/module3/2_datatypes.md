---
sidebar_position: 2
---

# Data Types

:::info

By convention, variables in Rust use snake case `_` instead of camel case

:::

## Strings

Strings can be a little bit confusing when you first start learning about Rust. You will typically see them defined in two different ways: 

### `&str`

This is a string slice. It is a view into a string, or a reference to a string. A string slice is a view into a string, or a reference to a string. It's fixed size that's known at compile time, It's does not require heap allocation. Comparable with Span in .NET

### `String`

A string is a heap-allocated string. It's growable, and can be modified

```rust showLineNumbers
let str_example: &str = "Hello";
println!("{}", str_example);

let string_example: String = "Hello".to_string();
println!("{}", string_example);
```

:::important

If you set a variable equal to a string without setting the type, e.g. `let string = "hello"` it will default to a `&str`. If you need a growable string you can use the `to_string()` function.

:::

## Integers and Decimals

Rust includes a large number of numeric types, and you'll get very used to setting both the type (integer, floating point) but also setting the size in bytes (`i32`, `f64`). This affects how many numbers your type can represent, and if it can handle negative numbers.

```rust showLineNumbers
let integer_example: i32 = 10;
println!("{}", str_example);

let float_example: f32 = 10.0;
println!("{}", float_example);
```

If you need to convert between a 32 bit and a 64 bit number you will **always** need to be explicit about the conversion.

:::info

The Rust compiler will try to infer the type automatically based on context.

:::

| Types | Description |
|----------|----------|
| i8, i16, i32, i64    | Signed integers ranging from 8 bit to 64 bit  |
| u8, u16, u32, u64    | Unsigned integers ranging from 8 bit to 64 bit  |
| f32, f64    | Floating point numbers in 32 and 64-bit variants  |
| isize, usize    | Integers that assume the CPU's native width. For example, in a 64 bit CPU both of these types will be 64-bits wide |

:::info

If you need to compare numeric types, you can use all the same syntax you would in .NET. Less than `<`, greater than `>`, equal to `==`, not equal to `!=`.

:::

## Arrays

```rust showLineNumbers
// Arrays are fixed size in Rust, and the size is declared at initialization
let array_example: [i32; 3] = [1, 2, 3];
println!("{}", array_example.len());

// Vecs are growable, heap-allocated arrays. They are the most commonly used collection in Rust.
// Vecs are similar to Lists in C#
// to add values to a Vec, you use the push method and the vec itself must be mutable
let vec_example: Vec<i32> = vec![1, 2, 3];
println!("{}", vec_example.len());
```

## Booleans

```rust showLineNumbers
let bool_example: bool = true;
println!("{}", bool_example);
```

Another interesting thing about Rust is that `null` does not exist. Yep, you read that right. No more `Object reference not set to the instance of an object` errors.

But how do you model a variable that may or may not have a value. That's where the `Option` enum comes in:

## Control Flow

### `if/else`

In Rust, conditional statements work similarly to C#, but with some important differences:

```rust showLineNumbers
// Basic if/else - notice no parentheses needed around condition
let number = 5;

if number < 10 {
    println!("Number is less than 10");
} else if number == 10 {
    println!("Number is exactly 10");
} else {
    println!("Number is greater than 10");
}
```

Unlike C#, Rust doesn't have a ternary operator (`condition ? then : else`). Instead, `if/else` can be used as an expression that returns a value:

```rust showLineNumbers
let number = 5;
let message = if number < 10 {
    "Number is less than 10"
} else {
    "Number is 10 or greater"
};

println!("{}", message);
```

When using `if` as an expression, all branches must return the same type. This won't compile:

```rust showLineNumbers
// This will not compile
let result = if number < 10 {
    "Less than 10"  // Returns &str
} else {
    5  // Returns i32
};
```

### `for x in y`

The `for` loop in Rust provides a safe and concise way to iterate over elements of a collection or a range of values. It's similar to C#'s `foreach` loop but with some Rust-specific features.

### Basic Syntax

```rust showLineNumbers
// Iterate over a range from 0 to 4 (5 is exclusive)
for i in 0..5 {
    println!("{}", i); // Prints 0, 1, 2, 3, 4
}

// Iterate over a range from 0 to 5 (inclusive)
for i in 0..=5 {
    println!("{}", i); // Prints 0, 1, 2, 3, 4, 5
}

// Iterate over elements in a collection
let numbers = vec![1, 2, 3, 4, 5];
for num in numbers {
    println!("{}", num);
}
```

The range syntax `start..end` creates a range that includes `start` but excludes `end`. If you want an inclusive range, you can use `start..=end`.

### `loop`

The `loop` keyword in Rust creates an infinite loop that continues until explicitly broken. This is Rust's most basic looping construct and differs from C#'s approach where you'd typically use `while(true)`.

#### Basic Syntax

```rust showLineNumbers
// Basic infinite loop
loop {
    println!("This will run forever unless broken");
    
    // Use break to exit the loop
    if some_condition {
        break;
    }
}
```

#### Named Loops and Values from Breaks

Rust allows you to label loops and return values from them when breaking:

```rust showLineNumbers
// A loop that computes a value
let result = 'calculation: loop {
    // Some computation
    
    if condition_met {
        break 'calculation computed_value;  // Returns value from the loop
    }
}

// Nested loops with labels
'outer: loop {
    println!("Outer loop");
    
    'inner: loop {
        println!("Inner loop");
        
        break 'outer;  // Breaks out of the outer loop
    }
    
    println!("This won't be reached");
}
```

This powerful construct makes certain algorithms more straightforward to express than in C# where you might need additional boolean flags or more complex control flow.

## The Option Type

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

match optional_string {
    Some(string_value) => println!("{}", string_value),
    None => println!("The optional string variable does not have a value")
}

```

This approach forces you to consider the case where a value might be absent, preventing many common bugs related to null references. The compiler won't let you use an `Option` as if it were definitely a T - you must handle both possibilities.

## Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. It's much more powerful than C#'s switch statement.

### Basic Match Syntax

```rust showLineNumbers
let number = 5;

match number {
    1 => println!("One!"),
    2 => println!("Two!"),
    3 => println!("Three!"),
    4 | 5 => println!("Four or five!"), // Multiple values
    6..=10 => println!("Six through ten"), // Range
    _ => println!("Something else"), // Default case
}
```

One of the reasons this is so powerful is that your code won't compile if you don't handle all edge cases. In the above example, if you were to remove the default case at the bottom the code wouldn't compile, because you're not handling all possible cases that a number could be.