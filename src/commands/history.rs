use crate::{utilities, read_history};
use std::process;

pub fn list_history(path: &str) {
    let history = read_history(&path);
    
    if history.is_empty() {
        println!("Command history is empty.");
    } else {
        for cmd in history {
            println!("{}", cmd);
        }
    }
}

pub fn clear_history(path: &str) {
    if let Err(e) = std::fs::write(&path, "") {
        utilities::print_error(&format!("Could not clear history: {}", e));
        process::exit(1);
    }
    println!("History cleared.");
    return;
}

pub fn remove_command(path: &str, _command: &str) {
    let command_list = std::fs::read_to_string(&path);
    if command_list.is_err() {
        utilities::print_error(&"Could not delete the command from the history".to_string());
    }
}
