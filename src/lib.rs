use std::time::Duration;

#[derive(Debug)]
pub enum State {
    Start,
    Stop,
}

pub use State::{Start, Stop};


#[derive(Debug)]
pub struct Project {
    name: String,
    duration: Duration,
}

impl Project {
    pub fn new(name: String) -> Self {
        let duration = Duration::new(0, 0);
        Project { name, duration }
    }
}

