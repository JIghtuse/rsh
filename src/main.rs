use std::io;
use std::io::prelude::*;
use std::process::Command;

fn execute_command(line: String) {
    let mut words = line.split_whitespace();

    let mut command = Command::new(words.next().unwrap());
    for word in words {
        command.arg(word);
    }

    command.spawn();
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        execute_command(line.unwrap());
    }
}
