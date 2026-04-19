use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{self, UNIX_EPOCH};
use std::fs;
use std::error::Error;
use std::io::ErrorKind;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum State {
    Start,
    Stop,
}

pub use State::{Start, Stop};

#[derive(Debug)]
pub struct Data {
    pub name: String,
    pub state: State,
    pub timestamp: u64,
}

impl Data {
    pub fn new(name: String, state: State, timestamp: u64) -> Self {
        Data {name, state, timestamp}
    }
}

pub fn read_database() -> Vec<Data> {
    let file = match fs::read_to_string("database.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                fs::File::create("database.txt").unwrap();
                return Vec::new();
            },
            _ => panic!("{error:?}"),
        }
    };
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

pub fn filter_for(project_name: &str) -> Vec<Data> {
    let database = read_database();
    let mut filtered_database: Vec<Data> = database.into_iter().filter(|data| data.name == project_name).collect();
    filtered_database
}

pub fn write_timestamp(project_name: &str, state: State) -> Result<(), Box<dyn Error>>{
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("database.txt")?;
    let timestamp = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    write!(&mut file, "{}\t{:?}\t{}\n", project_name, state, timestamp);
    Ok(())
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
