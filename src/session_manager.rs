use std::collections::HashMap;
use std::net::Ipv6Addr;
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Ipv6Addr>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_or_create(&self, session_id: &str, generator: impl FnOnce() -> Ipv6Addr) -> Ipv6Addr {
        let mut sessions = self.sessions.lock().unwrap();
        if !sessions.contains_key(session_id) {
            sessions.insert(session_id.to_string(), generator());
        }
        *sessions.get(session_id).unwrap()
    }
}