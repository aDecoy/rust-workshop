---
sidebar_position: 4
---

# Enums

When you work with Rust, you'll find that enums are much more powerful than in .NET. In Rust, enums can have properties:

```rust showLineNumbers
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

You can also add functions to your enums. Yep, you read that right.. Enums can have logic attached to them:

```rust showLineNumbers
enum User {
    Standard{user_details: UserDetails},
    Premium{user_details: UserDetails, is_premium: bool},
}

impl User {
    // When you create a user, you default it to a Standard User
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

## Understanding `match` and `if let`

### The `match` Keyword

The `match` keyword in Rust is similar to a switch statement in C#, but much more powerful. When you use `match`, you:

1. Take a value (like an enum variant)
2. Compare it against patterns
3. Execute code for the pattern that matches

In the example above, you see:

```rust showLineNumbers
let users_age = match &self {
    User::Standard { user_details } => user_details.age,
    User::Premium { user_details, is_premium: _ } => user_details.age
};
```

This code is:
- Matching against the enum variants of User
- Extracting the `user_details` field from each variant
- Using the `_` pattern to ignore the `is_premium` field
- Returning the `age` field from the matched variant

Unlike C# switches, Rust's `match` requires you to handle all possible cases, making your code safer and more complete.

### The `if let` Keyword

The `if let` syntax is a more concise way to handle a single pattern match. You'll find it particularly useful when you only care about one specific pattern and want to ignore all others.

In the example:

```rust showLineNumbers
if let Some(age) = users_age {
    println!("I'm {} years old.", age);
} else {  
    println!("I don't know my age.");
} 
```

This code:
1. Checks if `users_age` matches the pattern `Some(age)`
2. If it matches, binds the value inside `Some` to the variable `age`
3. Executes the first block of code if there's a match
4. Executes the `else` block if there's no match

This is much more concise than writing:

```rust showLineNumbers
match users_age {
    Some(age) => println!("I'm {} years old.", age),
    None => println!("I don't know my age."),
}
```

You'll use `if let` frequently when working with `Option` types to check if a value exists and extract it in one step.

:::info

`match` , `if` and `if let` all return values. Which means you can directly set a variable to the result of an `if` or `match` block:

```rust showLineNumbers

let my_new_variable = if condition == true {
    "The condition was true"
} else {
    "The condition was false"
}

println!(my_new_variable);

```

For this to work, all branches need to return the same value.

:::
