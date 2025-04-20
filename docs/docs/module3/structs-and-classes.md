---
sidebar_position: 3
---

# Structs and Classes

In Rust, you'll use structs as the primary way to organize related data, similar to how you'd use classes in .NET:

```rs
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

And now we get on to one of the most powerful data types in the Rust ecosystem, something that might not get the same praise in .NET. The `Enum`

## Enums

When you're working with Rust, you'll find that enums are much more powerful than in .NET. In Rust, enums can have properties:

```rs
struct UserDetails {
    email_address: String,
    age: Option<i32>,
    name: String,
}

enum User {
    Standard{user_details: UserDetails},
    Premium{user_details: UserDetails, is_premium: bool},
}
```

You can also add functions to your enums:

```rs
enum User {
    Standard{user_details: UserDetails},
    Premium{user_details: UserDetails, is_premium: bool},
}

impl User {
    // When you create a new user, you default it to a Standard User
    fn new(email_address: &str, name: &str) -> User {
        User::Standard { user_details: UserDetails {
            email_address: email_address.to_string(), name: name.to_string(), age: None
        } }
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn whats_my_age(&self) {
        // Everything in Rust returns a value, so you can assign a variable to the result of a match
        let users_age = match &self {
            User::Standard { user_details } => user_details.age,
            User::Premium { user_details, is_premium: _ } => user_details.age
        };

        // If let allows you to assign a variable and have an if condition in a single line
        if let Some(age) = users_age {
            println!("I'm {} years old.", age);
        } else {  
            println!("I don't know my age.");
        } 
    }
}
```

Explain the `match` keyword and the `if let` keywords.