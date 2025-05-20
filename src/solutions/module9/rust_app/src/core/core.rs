use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use regex::Regex;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("the provider password is incorrect")]
    IncorrectPassword,
    #[error("error interacting with database {0}")]
    DatabaseError(String),
    #[error("unexpected application error {0}")]
    ApplicationError(String),
}

#[async_trait::async_trait]
pub trait DataAccess: Send + Sync {
    async fn with_email_address(&self, email_address: &str) -> Result<User, ApplicationError>;
    async fn store(&self, user: User) -> Result<(), ApplicationError>;
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

#[derive(Serialize, Clone)]
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

impl User {
    // no 'self' at all defines a static method. Called using User::new()
    pub fn new(email_address: &str, name: &str, password: &str) -> Result<User, ApplicationError> {
        User::email_is_valid(email_address)?;
        User::password_is_valid(password)?;
        
        Ok(User::Standard {
            user_details: UserDetails {
                email_address: email_address.to_string(),
                name: name.to_string(),
                age: None,
                password: User::hash(password)?,
            },
        })
    }

    pub fn from(email_address: &str, name: &str, hashed_password: &str) -> User {
        User::Standard {
            user_details: UserDetails {
                email_address: email_address.to_string(),
                name: name.to_string(),
                age: None,
                password: hashed_password.to_string(),
            },
        }
    }

    fn hash(password: &str) -> Result<String, ApplicationError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|_| ApplicationError::ApplicationError("Failed to hash password".to_string()))?;

        Ok(hash.to_string())
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
    
    pub fn name(&self) -> String {
        match self {
            User::Standard { user_details } => user_details.name.clone(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.name.clone(),
        }
    }
    
    pub fn password(&self) -> String {
        match self {
            User::Standard { user_details } => user_details.password.clone(),
            User::Premium {
                user_details,
                is_premium: _,
            } => user_details.password.clone(),
        }
    }

    // &mut self is used because you want to mutate the data in this instance of the struct
    #[allow(dead_code)]
    fn update_name(&mut self, new_name: &str) {
        let user_details = match self {
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

    #[allow(dead_code)]
    fn update_age(&mut self, new_age: i32) {
        let user_details = match self {
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
    #[allow(dead_code)]
    fn update_to_premium(self) -> User {
        match self {
            User::Standard { user_details } => User::Premium {
                user_details,
                is_premium: true,
            },
            User::Premium { .. } => self,
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<(), ApplicationError> {
        let users_password = &self.password().clone();
        
        let parsed_hash = PasswordHash::new(users_password).map_err(|_| ApplicationError::ApplicationError("Failed to parse password hash".to_string()))?;
        
        let verified_password = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash);
        
        match verified_password {
            Ok(_) => Ok(()),
            Err(_) => Err(ApplicationError::IncorrectPassword)
        } 
    }

    fn password_is_valid(password: &str) -> Result<(), ApplicationError> {
        if password.len() < 8 {
            return Err(ApplicationError::ApplicationError("Password must be at least 8 characters long".to_string()));
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(ApplicationError::ApplicationError("Password must contain at least one uppercase letter".to_string()));
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(ApplicationError::ApplicationError("Password must contain at least one lowercase letter".to_string()));
        }
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(ApplicationError::ApplicationError("Password must contain at least one digit".to_string()));
        }
        Ok(())
    }
    
    fn email_is_valid(input: &str) -> Result<(), ApplicationError> {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if re.is_match(input) {
            Ok(())
        } else {
            Err(ApplicationError::ApplicationError("Invalid email address".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_new_user_is_created_should_be_standard() {
        let user = User::new("test@test.com", "James", "James!23").unwrap();
        
        if let User::Standard { user_details } = user {
            assert_eq!(user_details.email_address, "test@test.com");
            assert_eq!(user_details.name, "James");
        } else {
            panic!("Expected User::Standard variant");
        }
    }

    #[test]
    fn when_user_is_updated_to_premium_should_be_premium_user() {
        let user = User::new("test@test.com", "James", "James!23").unwrap();
        
        let premium_user = user.update_to_premium();

        if let User::Premium { user_details, is_premium } = premium_user {
            assert_eq!(user_details.email_address, "test@test.com");
            assert_eq!(user_details.name, "James");
        } else {
            panic!("Expected User::Standard variant");
        }
    }

    #[test]
    fn when_a_user_is_created_should_be_able_to_update_age() {
        let mut user = User::new("test@test.com", "James", "James!23").unwrap();

        assert_eq!(user.details().age, None);
        
        user.update_age(10);

        assert_eq!(user.details().age.unwrap(), 10);
    }

    #[test]
    fn when_a_user_is_created_should_be_able_to_update_name() {
        let mut user = User::new("test@test.com", "James", "James!23").unwrap();

        assert_eq!(user.details().name, "James");
        
        user.update_name("John");

        assert_eq!(user.details().name, "John");
    }

    #[test]
    fn when_user_is_created_with_an_invalid_email_should_return_error() {
        let user = User::new("thisisaninvalidemail", "James", "James!23");

        assert!(user.is_err());
    }

    #[test]
    fn when_user_is_created_with_an_invalid_password_should_return_error() {
        let user = User::new("test@test.com", "James", "james");

        assert!(user.is_err());
    }

    #[test]
    fn when_user_is_created_should_verify_a_matching_password() {
        let user = User::new("test@test.com", "James", "James!23").unwrap();
        
        assert_ne!(user.password(), "Test!23");
        
        let is_password_valid = user.verify_password("James!23");
        
        assert!(is_password_valid.is_ok());
    }

    #[test]
    fn when_user_is_created_should_fail_if_password_does_not_match() {
        let user = User::new("test@test.com", "James", "James!23").unwrap();

        assert_ne!(user.password(), "Test!23");

        let is_password_valid = user.verify_password("This is the wrong password");

        assert!(is_password_valid.is_err());
    }
}
