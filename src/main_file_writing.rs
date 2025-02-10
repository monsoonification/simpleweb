extern crate actix_web; 
extern crate chrono;

use std::io::prelude::*;
use std::fs::{File,OpenOptions};
use std::io;
use actix_web::cookie::time::ext;
use actix_web::{web, App, HttpServer, Responder};
use chrono::*;


// fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
//     let mut f = File::create(filename)?;
//     (f.write_all(string))?;
//     Ok(())
// }

fn formatted_time_entry() -> String {
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

async fn log_time() -> impl Responder {
    let entry = formatted_time_entry();
    let entry_bytes = entry.as_bytes();
    if let Err(e) = record_entry_in_log("log.txt", &entry_bytes) {
        return format!("Error: {}", e);
    }
    format!("Entry logged!: {}", entry)
}

// fn do_log_time() -> String {
//     match log_time("log.txt") {
//         Ok(entry) => format!("Entry logged!: {}", entry),
//         Err(e) => format!("Error: {}", e)
//     }
// }

async fn do_log_time() -> impl Responder {
    "Logging time..."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(log_time))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}