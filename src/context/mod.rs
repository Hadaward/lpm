use std::sync::Arc;
use tokio::sync::Mutex;
use self::state::State;

pub mod state;

fn wrapped_state<T>(value: T) -> Arc<Mutex<State<T>>> where T: Clone {
    Arc::new(Mutex::new(State::new(value)))
}

#[derive(Clone)]
pub struct Context {
    pub home_dir: Arc<Mutex<State<String>>>,
    pub download_dir: Arc<Mutex<State<String>>>,
    pub cache_dir: Arc<Mutex<State<String>>>,
    pub homeref_dir: Arc<Mutex<State<String>>>
}

impl Context {
    pub fn new() -> Self {
        Self {
            home_dir: wrapped_state(String::new()),
            download_dir: wrapped_state(String::new()),
            cache_dir: wrapped_state(String::new()),
            homeref_dir: wrapped_state(String::new())
        }
    }
}