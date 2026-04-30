use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::time::Duration;
use std::time::{self, UNIX_EPOCH};

#[derive(PartialEq, Debug)]
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
        Data {
            name,
            state,
            timestamp,
        }
    }
}

pub struct TimeBlock {
    pub start_timestamp: u64,
    pub stop_timestamp: u64,
    pub secs: u64,
}

impl fmt::Display for TimeBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut h = 0;
        let mut m = 0;
        let mut s = 0;

        for _ in 0..self.secs {
            s += 1;
            if s > 59 {
                s = 0;
                m += 1;
            }
            if m > 59 {
                m = 0;
                h += 1;
            }
        }
        write!(f, "{h}hours {m}minutes {s}seconds")
    }
}

pub fn read_database() -> Vec<Data> {
    let file = match fs::read_to_string("database.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                fs::File::create("database.txt").unwrap();
                return Vec::new();
            }
            _ => panic!("{error:?}"),
        },
    };
    let results: Vec<Data> = file
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("\t").collect();
            let name = v[0].to_string();
            let state = match &*v[1] {
                "Start" => Start,
                "Stop" => Stop,
                _ => Start,
            };
            let timestamp = v[2].parse::<u64>().unwrap();
            Data {
                name,
                state,
                timestamp,
            }
        })
        .collect();
    results
}

pub fn filter_for(project_name: &str) -> Vec<Data> {
    let database = read_database();
    let mut filtered_database: Vec<Data> = database
        .into_iter()
        .filter(|data| data.name == project_name)
        .collect();
    filtered_database
}

pub fn write_timestamp(project_name: &str, state: State) -> Result<(), Box<dyn Error>> {
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

pub fn get_timeblocks(project_name: &str) -> Vec<TimeBlock> {
    let entries = filter_for(&project_name);
    let mut timeblocks: Vec<TimeBlock> = Vec::new();
    if entries.len() % 2 == 0 && entries.len() != 0 {
        for i in 0..entries.len() {
            if i % 2 == 0 {
                let start_timestamp = entries[i].timestamp;
                let stop_timestamp = entries[i + 1].timestamp;
                let secs = entries[i + 1].timestamp - entries[i].timestamp;
                timeblocks.push(TimeBlock {
                    start_timestamp,
                    stop_timestamp,
                    secs,
                })
            } else {
                continue;
            }
        }
        timeblocks
    } else if entries.len() % 2 != 0 && entries.len() > 1 {
        let range = entries.len() - 1;
        for i in 0..range {
            if i % 2 == 0 {
                let start_timestamp = entries[i].timestamp;
                let stop_timestamp = entries[i + 1].timestamp;
                let secs = entries[i + 1].timestamp - entries[i].timestamp;
                timeblocks.push(TimeBlock {
                    start_timestamp,
                    stop_timestamp,
                    secs,
                })
            } else {
                continue;
            }
        }
        timeblocks

    } else {
        Vec::new()
    }
}

pub fn show_time(secs: u64) {
    let mut h = 0;
    let mut m = 0;
    let mut s = 0;

    for _ in 0..secs {
        s += 1;
        if s > 59 {
            s = 0;
            m += 1;
        }
        if m > 59 {
            m = 0;
            h += 1;
        }
    }
    println!("{h}hours {m}minutes {s}seconds");
}
//fn data_parse
