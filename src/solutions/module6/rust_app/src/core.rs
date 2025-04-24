use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
// This struct is available everywhere (pub)
pub struct RegisterUserRequest {
    // And the properties are public
    pub email_address: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email_address: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
// This struct is available everywhere (pub)
pub struct UserDetails {
    // But the properties are private
    email_address: String,
    password: String,
    age: Option<i32>,
    name: String,
}

#[derive(Clone)]
pub enum User {
    Standard {
        user_details: UserDetails,
    },
    Premium {
        user_details: UserDetails,
        is_premium: bool,
    },
}

impl User {
    // Functions also need to be defined as public
    pub fn new(email_address: &str, name: &str, password: &str) -> User {
        User::Standard {
            user_details: UserDetails {
                email_address: email_address.to_string(),
                name: name.to_string(),
                age: None,
                password: password.to_string(),
            },
        }
    }
    pub fn details(&self) -> &UserDetails {
        match self {
            User::Standard { user_details } => user_details,
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details,
        }
    }
    pub fn email_address(&self) -> String {
        match self {
            User::Standard { user_details } => user_details.email_address.clone(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.email_address.clone(),
        }
    }
    
    fn update_name(&mut self, new_name: &str) {
        let mut user_details = match self {
            // The '*' is used to dereference the value of the variable, so you can change it.
            // De-referncing refers to accessing the underlying value the reference points to
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

    // Semantics for getters/setters use Java like syntax where you would
    // have a function for getting and setting properties
    pub fn age(&self) -> Option<i32> {
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

    pub fn verify_password(&self, password: &str) -> Result<(), ()> {
        let user_password = match &self {
            User::Standard { user_details } => user_details.password.as_str(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.password.as_str(),
        };

        match user_password == password {
            true => Ok(()),
            false => Err(()),
        }
    }
}
