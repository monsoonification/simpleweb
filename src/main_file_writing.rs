extern crate chrono;

use std::io::prelude::*;
use std::fs::{File,OpenOptions};
// use std::fs::File;
use std::io;
use chrono::*;


fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = File::create(filename)?;
    (f.write_all(string))?;
    Ok(())
}

fn log_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = (OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename))?;
    (file.write_all(bytes))?;
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = log_time_entry();
    let entry_bytes = entry.as_bytes();
    record_entry_in_log(filename, entry_bytes)?;
    Ok(())
}


fn main() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(e) => println!("Error: {}", e)
    }
}