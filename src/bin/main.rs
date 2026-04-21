use std::env;
use std::error::Error;
use std::time::Duration;
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
    println!("here have a look at the database:");
    for data in get_timeblocks(&args[2]) {
        println!("{}", data);
    }
    
    println!("\ntotal time spent on project {}:",&args[2]);
    let mut total_time: u64 = 0;
    for timeblock in get_timeblocks(&args[2]) {
        total_time += timeblock.secs;
    }
    show_time(total_time);
    Ok(())
}
