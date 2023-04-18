#![windows_subsystem = "windows"]

use std::{env, process, thread};
use std::time::Duration;
use steamworks::{AppId, Client};

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  if 1 > args.len() {
    println!("The application needs 1 argument.");
    process::exit(0);
  }

  let app_id = match args[0].parse::<u32>() {
    Err(_) => {
      println!("Invalid number!");
      process::exit(0);
    }
    Ok(it) => AppId::from(it)
  };

  Client::init_app(app_id).unwrap();
  loop {
    thread::sleep(Duration::from_secs(60));
  }
}