const HELP: &str = r#"
Usage: damn [OPTIONS] [COMMAND]
Options:
  -V, --version    Print version information
Commands:
  add <name>       Add a new command to the list
  danger <name>    Add a new dangerous command to the list
  remove <name>    Remove a command from the list
  list             List all commands
  clear            Clear the known command history
  help             Print help information
For more information, see <https://github.com/duartecaldascardoso/damn>
"#;

pub fn print_help() {
    println!("{}", HELP);
}

pub fn print_version() {
    let path: &'static str = env!("CARGO_PKG_VERSION");
    println!("damn {path}");
}

pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
}