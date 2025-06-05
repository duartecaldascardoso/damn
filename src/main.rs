mod commands;
mod suggestions;

use clap::Subcommand;
use clap::{CommandFactory, Parser};
use commands::{history, metrics, user_creation, utilities};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use crate::suggestions::similarity_suggest::suggest_command;

#[derive(Parser)]
#[command(
    name = "damn",
    version,
    about = "Your shell’s undo button: auto-suggest and rerun the right command based on your own shell history."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(trailing_var_arg = true)]
    failed_command: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Danger { name: String },
    Remove { name: String },
    List,
    Clear,
    Metrics,
}

fn read_history(path: &str) -> Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

fn main() {
    let cli = Cli::parse();

    let home = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            utilities::print_error("Could not determine HOME directory.");
            process::exit(1);
        }
    };
    let history_path = format!("{}/.damn_history", home);

    match &cli.command {
        Some(Commands::List) => {
            history::list_history(&history_path);
        }
        Some(Commands::Clear) => {
            history::clear_history(&history_path);
        }
        Some(Commands::Add { name }) => {
            user_creation::add_command(&history_path, name);
        }
        Some(Commands::Danger { name }) => {
            user_creation::add_dangerous_command(&history_path,name);
        }
        Some(Commands::Remove { name }) => {
            history::remove_command(&history_path, name);
        }
        Some(Commands::Metrics) => {
            metrics::usage_metrics(&history_path);
        }
        None => {
            if !cli.failed_command.is_empty() {
                let failed_command = cli.failed_command.join(" ");
                let history = read_history(&history_path);
                if let Some(suggestion) = suggest_command(&failed_command, &history) {
                    println!("{}", suggestion);
                } else {
                    println!("No similar command found in history.");
                    process::exit(1);
                }
            } else {
                Cli::command().print_help().unwrap();
                println!();
            }
        }
    }
}
