#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;
use steamworks::Client;

fn main() {
  Client::init().unwrap();
  loop {
    thread::sleep(Duration::from_secs(60));
  }
}