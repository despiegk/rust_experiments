#![feature(unix_chown)]
use std::env;
use std::os::unix::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    println!("Cmd {}", cmd);
    println!("Searching for {}", query);
    println!("In file {}", file_path);


    fs::chown("~/sandbox", Some(0), Some(0)).expect("Error");

}