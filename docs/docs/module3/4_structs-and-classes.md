---
sidebar_position: 3
---

# Structs and Classes

In Rust, you'll use structs as the primary way to organize related data, similar to how you'd use classes in .NET:

```rust showLineNumbers
// A struct is comparable to a class in .NET
struct User {
    email_address: String,
    name: String,
}

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

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> PremiumUser {
    }
}
```

:::info

By way of convention, the `constructor` of a struct is always implemented with an `fn new()` function. To create a new user, you would call `let my_new_user = User::new();`

You'll also notice there is no `return` keyword at the end of a function. The `;` indicates the end of a line, much like in C#. But if you leave the `;` off, the compiler assumes you want to return the value. The `new_user` variable demonstrates that in the above code sample.

The `return` keyword does still exist, and you can use that to return a value from a function early.

:::

And now we get on to one of the most powerful data types in the Rust ecosystem, something that might not get the same praise in .NET. The `Enum`