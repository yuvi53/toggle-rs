use std::time::Duration;

#[derive(Debug)]
pub enum State {
    Start,
    Stop,
}

pub use State::{Start, Stop};


#[derive(Debug)]
pub struct Data {
    name: String,
    state: State,
    timestamp: u64,
}

impl Data {
    pub fn new(name: String, state: State, timestamp: u64) -> Self {
        Data {name, state, timestamp}
    }
}



