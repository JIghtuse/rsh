use std::convert::From;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::env;
use std::str::SplitWhitespace;
use std::collections::HashMap;

#[derive(Debug)]
enum RshError {
    Io(io::Error)
}

impl From<io::Error> for RshError {
    fn from(e: io::Error) -> Self {
        RshError::Io(e)
    }
}

type BuiltinCommand = fn(&mut SplitWhitespace) -> Result<(), RshError>;

fn cd(paths: &mut SplitWhitespace) -> Result<(), RshError> {
    if let Some(path) = paths.next() {
        try!(env::set_current_dir(path));
    }
    Ok(())
}

fn pwd(_: &mut SplitWhitespace) -> Result<(), RshError> {
    let dir = try!(env::current_dir());
    println!("{}", dir.display());
    Ok(())
}

fn execute_command(line: String, builtins: &HashMap<&str, BuiltinCommand>) -> Result<(), RshError> {
    let mut words = line.split_whitespace();

    if let Some(program) = words.next() {
        if builtins.contains_key(program) {
            try!(builtins[program](&mut words));
        } else {
            let mut command = Command::new(program);
            for word in words {
                command.arg(word);
            }
            try!(command.spawn());
        }
    }
    Ok(())
}

fn main() {
    let mut builtins: HashMap<&str, BuiltinCommand> = HashMap::new();
    builtins.insert("cd", cd);
    builtins.insert("pwd", pwd);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match execute_command(line.unwrap(), &builtins) {
            Err(RshError::Io(e)) => println!("IO Error: {}", e),
            _ => ()
        }
    }
}
