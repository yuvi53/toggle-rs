use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{self, UNIX_EPOCH};
use toggle_rs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let project_name = &args[2];
    let state = match &*args[1] {
        "--start" => Start,
        "--stop" => Stop,
        _ => Start,
    };
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
