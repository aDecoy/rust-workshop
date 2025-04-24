use std::sync::Mutex;
use crate::core::{DataAccess, User};

pub struct InMemoryDataAccess {
    // Mutex is a type that provides safe concurrent access to a value
    users: Mutex<Vec<User>>,
}

impl InMemoryDataAccess {
    pub  fn new() -> InMemoryDataAccess {
        InMemoryDataAccess {
            users: Mutex::new(Vec::new()),
        }
    }
}

impl DataAccess for InMemoryDataAccess {
    fn with_email_address(&self, email_address: &str) -> Option<User> {
        // .lock() is a method on Mutex that locks the value 
        // inside the Mutex allowing it to be accessed safely 
        self.users.lock().unwrap().iter().find(|u| u.email_address() == email_address).cloned()
    }

    fn store(&self, user: User) {
        self.users.lock().unwrap().push(user);
    }
}