use crate::core::User;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct AppState {
    // Pub crate means the users property is available inside the crate
    // But if someone uses this as a library they won't get access to it
    pub(crate) users: Vec<User>,
}

pub type SharedState = Arc<RwLock<AppState>>;
