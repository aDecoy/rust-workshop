use std::sync::{Arc, RwLock};
use crate::core::User;

#[derive(Default)]
pub struct AppState {
    pub(crate) users: Vec<User>,
}

pub type SharedState = Arc<RwLock<AppState>>;