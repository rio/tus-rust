extern crate tus;
extern crate hyper;

use std::fs;
use std::env;
use std::process::{Command, Child};

pub fn create_temp_file(name: &str, size: u64) {
    let mut temp_file_path = env::temp_dir();
    temp_file_path.push(name);

    let temp_file = fs::File::create(temp_file_path).unwrap();
    temp_file.set_len(size).unwrap();
}

pub fn remove_temp_file(name: &str) {
    let mut temp_file_path = env::temp_dir();
    temp_file_path.push(name);

    fs::remove_file(temp_file_path).unwrap()
}

pub fn spawn_server() -> Child {
    Command::new("cargo").arg("run").spawn().unwrap()
}
