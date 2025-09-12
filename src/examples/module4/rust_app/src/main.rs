fn main() {
    // Mut keyword declares this variable as something that can be mutated in the future
    let mut user = User::new("james@eastham.com", "James");

    user.say_hello();

    // ERROR: cannot borrow as mutable
    user.update_name("John");

    user.say_hello();

    let mut premium_user = user.update_to_premium();

    // Calling say_hello here will cause an error because the original instance of user has been dropped
    // user.say_hello();
    premium_user.say_hello();

    premium_user.whats_my_age();

    premium_user.update_age(32);

    premium_user.whats_my_age();
}

struct UserDetails {
    email_address: String,
    age: Option<i32>,
    name: String,
}

enum User {
    Standard {
        user_details: UserDetails,
    },
    Premium {
        user_details: UserDetails,
        is_premium: bool,
    },
}

impl User {
    // When you create a new user, you default it to a Standard User
    fn new(email_address: &str, name: &str) -> User {
        User::Standard {
            user_details: UserDetails {
                email_address: email_address.to_string(),
                name: name.to_string(),
                age: None,
            },
        }
    }

    // When you want to update a value, you extract the user_details property
    // from the enum and then update the value.
    fn update_name(&mut self, new_name: &str) {
        let mut user_details = match self {
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        };

        user_details.name = new_name.to_string();
    }

    fn update_age(&mut self, new_age: i32) {
        let mut user_details = match self {
            // The '*' is used to dereference the value of the variable, so you can change it.
            // De-referncing refers to accessing the underlying value the reference points to
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        };

        user_details.age = Some(new_age);
    }

    // When referencing a value you extract the value again
    // and then implement it.
    fn say_hello(&self) {
        let name = match &self {
            User::Standard { user_details } => user_details.name.as_str(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.name.as_str(),
        };

        // String interpolation
        println!("Hello! I'm {}. I'm a standard user.", name);
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn get_age(&self) -> Option<i32> {
        let age = match &self {
            User::Standard { user_details } => user_details.age,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.age,
        };

        age
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn whats_my_age(&self) {
        // Everything in Rust returns a value, so you can assign a variable to the result of a match
        let users_age = match &self {
            User::Standard { user_details } => user_details.age,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.age,
        };

        // If let allows you to assign a variable and have an if condition in a single line
        if let Some(age) = users_age {
            println!("I'm {} years old.", age);
        } else {
            println!("I don't know my age.");
        }
    }

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> User {
        match self {
            User::Standard { user_details } => User::Premium {
                user_details,
                is_premium: true,
            },
            User::Premium { .. } => self,
        }
    }

    fn details(&self) -> &UserDetails {
        match self {
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        }
    }
}
