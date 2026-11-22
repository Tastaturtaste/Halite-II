use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use uuid::Uuid;

pub type SessionStore = Arc<RwLock<HashMap<String, String>>>;

pub fn new_store() -> SessionStore {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Creates a new session for `email` and returns its token.
pub fn create(sessions: &SessionStore, email: &str) -> String {
    let token = Uuid::new_v4().to_string();
    sessions
        .write()
        .expect("session lock poisoned")
        .insert(token.clone(), email.to_string());
    token
}

/// Returns the email belonging to `token`, if the session exists.
pub fn get(sessions: &SessionStore, token: &str) -> Option<String> {
    sessions
        .read()
        .expect("session lock poisoned")
        .get(token)
        .cloned()
}

/// Removes the session for `token`.
pub fn remove(sessions: &SessionStore, token: &str) -> bool {
    sessions
        .write()
        .expect("session lock poisoned")
        .remove(token)
        .is_some()
}
