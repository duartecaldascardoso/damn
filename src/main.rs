mod help;
mod commands;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process;
use strsim::levenshtein;
use crate::commands::history;

fn read_history(path: &str) -> Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

fn similarity(a: &str, b: &str) -> f64 {
    let dist = levenshtein(a, b) as f64;
    let max_len = a.len().max(b.len()) as f64;
    if max_len == 0.0 {
        1.0
    } else {
        1.0 - (dist / max_len)
    }
}

fn suggest_command<'a>(failed: &'a str, history: &'a [String]) -> Option<&'a String> {
    history
        .iter()
        .filter(|cmd| similarity(failed, cmd) > 0.99)
        .max_by(|a, b| {
            let sim_a = similarity(failed, a);
            let sim_b = similarity(failed, b);
            sim_a.partial_cmp(&sim_b).unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let home = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            help::print_error("Could not determine HOME directory.");
            process::exit(1);
        }
    };
    let history_path = format!("{}/.damn_history", home);

    if args.len() <= 1 {
        help::print_help();
        return;
    }

    // Handle flags before subcommands
    match args[1].as_str() {
        "--help" | "-h" | "help" => {
            help::print_help();
            return;
        }
        "--version" | "-V" => {
            help::print_version();
            return;
        }
        _ => {}
    }

    match args[1].as_str() {
        "list" => {
            history::list_history(&history_path);
        }
        "clear" => {
            history::clear_history(&history_path);
        }
        "add" => {
            if let Some(name) = args.get(2) {
                let mut file = match OpenOptions::new().append(true).create(true).open(&history_path) {
                    Ok(f) => f,
                    Err(e) => {
                        help::print_error(&format!("Could not open history file: {}", e));
                        process::exit(1);
                    }
                };
                if let Err(e) = writeln!(file, "{}", name) {
                    help::print_error(&format!("Could not write to history file: {}", e));
                    process::exit(1);
                }
                println!("Added: {}", name);
            } else {
                help::print_error("No command provided to add.");
                process::exit(1);
            }
            return;
        }
        "remove" => {
            history::remove_command(&history_path, args.get(2).unwrap());
        }
        _ => {
            let failed_command = &args[1];
            let history = read_history(&history_path);
            if let Some(suggestion) = suggest_command(failed_command, &history) {
                println!("{}", suggestion);
            } else {
                process::exit(1);
            }
        }
    }
}