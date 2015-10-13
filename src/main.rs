use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::env;

fn execute_command(line: String) {
    let mut words = line.split_whitespace();

    if let Some(program) = words.next() {
        if program == "cd" {
            if let Some(path) = words.next() {
                env::set_current_dir(path).unwrap();
            }
            return;
        }

        let mut command = Command::new(program);
        for word in words {
            command.arg(word);
        }

        command.spawn();
    }
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        execute_command(line.unwrap());
    }
}
