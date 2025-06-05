use crate::commands::utilities;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

pub fn add_command(path: &str, command_name: &str) {
    let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(f) => f,
        Err(e) => {
            utilities::print_error(&format!("Could not open history file: {}", e));
            process::exit(1);
        }
    };
    if let Err(e) = writeln!(file, "{}", command_name) {
        utilities::print_error(&format!("Could not write to history file: {}", e));
        process::exit(1);
    }
    println!("Added: {}", command_name);
}

pub fn add_dangerous_command(_path: &str, _command_name: &str) {}
