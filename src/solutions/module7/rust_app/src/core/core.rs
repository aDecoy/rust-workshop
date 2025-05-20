use serde::{Deserialize, Serialize};

pub trait DataAccess: Send + Sync {
    fn with_email_address(&self, email_address: &str) -> Option<User>;
    fn store(&self, user: User);
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
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

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
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

#[derive(Serialize)]
pub struct UserDto {
    email_address: String,
    name: String,
    age: Option<i32>,
    is_premium: bool,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        match user {
            User::Standard { user_details } => UserDto {
                email_address: user_details.email_address.clone(),
                name: user_details.name.clone(),
                age: user_details.age,
                is_premium: false,
            },
            User::Premium {
                user_details,
                is_premium: _,
            } => UserDto {
                email_address: user_details.email_address.clone(),
                name: user_details.name.clone(),
                age: user_details.age,
                is_premium: true,
            },
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::Standard {
            user_details: UserDetails::default(),
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            User::Standard { user_details } => {
                write!(f, "Standard User: {}", user_details.email_address)
            }
            User::Premium {
                user_details,
                is_premium: _,
            } => write!(f, "Premium User: {}", user_details.email_address),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                User::Standard { user_details },
                User::Standard {
                    user_details: other_user_details,
                },
            ) => user_details.email_address == other_user_details.email_address,
            (
                User::Premium {
                    user_details,
                    is_premium: _,
                },
                User::Premium {
                    user_details: other_user_details,
                    is_premium: other_is_premium,
                },
            ) => user_details.email_address == other_user_details.email_address,
            _ => false,
        }
    }
}

impl User {
    // no 'self' at all defines a static method. Called using User::new()
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

    // &mut self is used because you want to mutate the data in this instance of the struct
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
