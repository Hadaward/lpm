use std::sync::Arc;

use tokio::sync::Mutex;

use super::Version;

#[derive(Clone)]
pub struct Context {
    pub lua_versions: Arc<Mutex<Vec<Version>>>,
    pub lua_versions_updated_at: Arc<Mutex<State<u128>>>,
    pub lpm_home: Arc<Mutex<State<String>>>,
    pub lpm_downloads_dir: Arc<Mutex<State<String>>>
}

impl Context {
    pub fn new() -> Self {
        Self {
            lua_versions: Arc::new(Mutex::new(Vec::new())),
            lua_versions_updated_at: Arc::new(Mutex::new(State::new(0))),
            lpm_home: Arc::new(Mutex::new(State::new(String::new()))),
            lpm_downloads_dir: Arc::new(Mutex::new(State::new(String::new())))
        }
    }
}

#[derive(Debug, Clone)]
pub struct State<T: Clone> {
    value: T
}

impl<T> State<T> where T: Clone {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }

    pub fn get(&self) -> T {
        return self.value.clone();
    }

    pub fn set(&mut self, value: T) where T: Clone {
        self.value = value;
    }
}