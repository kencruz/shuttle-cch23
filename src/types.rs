use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Clone)]
pub struct AppState {
    pub record_last_updated: Arc<Mutex<HashMap<String, u64>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            record_last_updated: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}
