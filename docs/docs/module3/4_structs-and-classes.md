---
sidebar_position: 3
---

# Structs and Classes

In Rust, you'll use `struct` as the primary way to organize related data, similar to how you'd use classes in .NET:

:::important
Although structs might *feel* like classes in .NET, Rust does not have a concept of inheritence.
:::


```rust showLineNumbers
// A struct is comparable to a class in .NET
struct User {
    email_address: String,
    name: String,
}
```

## The `impl` block

```rust showLineNumbers
// A key difference is that the methods/functions of your struct
// Are separate from the actual definition. Inside this `impl` block
impl User {
    // no 'self' at all defines a static method. Called using User::new()
    fn new(email_address: &str, name: &str) -> User {
        let new_user = User {
            email_address: email_address.to_string(), name: name.to_string(), age: None
        };

        new_user
    }
    
    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_email_address(&mut self, email_address: &str) {
    }

    // &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
    fn say_hello(&self) {
    }

    // Using 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> PremiumUser {
    }
}
```

In Rust, the functions of a struct are defined seperately to the `struct` itself. For that, you will use an `impl` block.

Unlike other languages, `new` is not a keyword in Rust. By way of convention, the `constructor` of a struct is always implemented with an `fn new()` function. To create a user, you would call `let my_new_user = User::new();`

You'll also notice there is no `return` keyword at the end of a function. The `;` indicates the end of a line, much like in C#. If you leave the `;` off, the compiler assumes you want to return the value. The `new_user` variable demonstrates that in the above code sample.

## What is `self`

`self` can be one of the more confusing aspects of functions that are defined on structs. After you understand the four different variations and what they all do, it's a little bit simpler.

### No `self` parameter

This creates a static method (similar to `static` in C#). These are called using the struct name, like `User::new()`.

```rust
// no 'self' at all defines a static method. Called using User::new()
fn new(email_address: &str, name: &str) -> User {

}
```

### `&self`

This is an immutable reference to the current instance of the struct. Use this when you need to read data from the struct but not modify it. This is the most common form and similar to methods in C# that don't modify the object's state.

```rust showLineNumbers
// &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
fn say_hello(&self) {
}
```

### `&mut self` 

This is a mutable reference to the instance. Use this when your method needs to modify the struct's data. This is similar to methods in C# that change the object's properties.

```rust showLineNumbers
// &mut self is used because you want to mutate the data in this instance of the struct
fn update_email_address(&mut self, email_address: &str) {
}
```

`&mut self` is an interesting case because if you don't declare the initial instance of a class as `mut`, then you won't be able to call any functions on that struct that need `&mut self`.

```rust showLineNumbers
let user = User::new("test@test.com", "James Eastham");
user.update_email_address("newemail@test.com");
```

:::danger
error[E0596]: cannot borrow `user` as mutable, as it is not declared as mutable
:::

### `self`

This takes ownership of the instance. When you use this, the original instance is consumed by the method. This is rare and typically used when transforming an object into something else, after which the original is no longer needed.

```rust showLineNumbers
// Using 'self' is a rare case where you want to take ownership of the original instance and use something new
// calling this function will prevent the original instance from being used, as this function
// takes ownership and then drop the original instance
fn update_to_premium(self) -> PremiumUser {
}
```

These different variations of `self` make Rust's ownership system explicit in your method signatures, clearly indicating how each method interacts with the struct's data.

The `return` keyword does still exist, and you can use that to return a value from a function early.
The `return` keyword does still exist, and you can use that to return a value from a function early.

And now you get on to one of the most powerful data types in the Rust ecosystem, something that might not get the same praise in .NET. The `Enum`