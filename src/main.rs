use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::env;
use std::str::SplitWhitespace;
use std::collections::HashMap;

type BuiltinCommand = fn(&mut SplitWhitespace);

fn cd(paths: &mut SplitWhitespace) {
    if let Some(path) = paths.next() {
        if let Err(error) = env::set_current_dir(path) {
            println!("Cannot change directory: {}", error)
        }
    }
}

fn pwd(_: &mut SplitWhitespace) {
    match env::current_dir() {
        Ok(current_dir) => println!("{}", current_dir.display()),
        Err(error) => println!("Cannot print current dir: {}", error),
    }
}

fn execute_command(line: String, builtins: &HashMap<&str, BuiltinCommand>) {
    let mut words = line.split_whitespace();

    if let Some(program) = words.next() {
        if builtins.contains_key(program) {
            builtins[program](&mut words);
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
    let mut builtins: HashMap<&str, BuiltinCommand> = HashMap::new();
    builtins.insert("cd", cd);
    builtins.insert("pwd", pwd);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        execute_command(line.unwrap(), &builtins);
    }
}
