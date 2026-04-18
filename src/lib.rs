use std::time::Duration;
use std::fs;

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

pub fn read_database() -> Vec<Data> {
    let file = fs::read_to_string("database.txt").unwrap();
    let results: Vec<Data> = file
        .lines()
        .map( |line| {
            let v: Vec<&str> = line.split("\t").collect();
            let name = v[0].to_string();
            let state = match &*v[1] {
                "Start" => Start,
                "Stop" => Stop,
                _ => Start,
            };
            let timestamp = v[2].parse::<u64>().unwrap();
            Data {name, state, timestamp}
        })
    .collect();
    results
}
    //sort the timestamp for the new frist for a given project
    //which brings as back to our first point(that we didn't mention, sorry for that!)to filter the regex with the
    //name given with the envocking argmuments

/*
fn next_after_read_database() {
     if vec.pop().unwrap().state == Start {
         //don't allow any new creation of the timestamp for the same project
    }
     else {
         // do allow creation of timestamp for the same project
    }
}
*/



