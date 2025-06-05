use std::fs::File;
use std::io::{BufRead, BufReader};
use strsim::levenshtein;

fn read_history(path: &str) -> Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

fn suggest_command<'a>(failed: &'a str, history: &'a [String]) -> Option<&'a String> {
    history
        .iter()
        .min_by_key(|cmd| levenshtein(failed, cmd))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let failed_command = &args[1];
    let home = std::env::var("HOME").expect("Could not get HOME directory");
    let history_path = format!("{}/.damn_history", home);
    let history = read_history(&history_path);
    if let Some(suggestion) = suggest_command(failed_command, &history) {
        println!("{}", suggestion);
    }
}