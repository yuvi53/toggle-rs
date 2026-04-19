use std::env;
use std::error::Error;
use toggle_rs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let project_name = &args[2];
    let state = match &*args[1] {
        "--start" => Start,
        "--stop" => Stop,
        _ => panic!("your state strings are not working!"),
    };
    let previous_state = match filter_for(&args[2]).pop() {
        Some(data) => data.state,
        None => Stop,
    };

    if state != previous_state {
        write_timestamp(&args[2], state)?;
    }
    println!("here have a look at the database:\n");
    for data in read_database() {
        println!("{:?}", data);
    }
    Ok(())
}
