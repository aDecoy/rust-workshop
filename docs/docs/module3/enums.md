---
sidebar_position: 4
---

# Enums

Enums can have properties

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

Enums can also have functions:

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