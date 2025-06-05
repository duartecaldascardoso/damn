use crate::read_history;
use std::collections::HashMap;

pub fn usage_metrics(path: &str) {
    let history = read_history(path);

    if history.is_empty() {
        println!("Command history is empty. No commands to analyze.");
        return;
    }

    let mut command_counts: HashMap<String, usize> = HashMap::new();
    for cmd in history {
        *command_counts.entry(cmd).or_insert(0) += 1;
    }

    let mut sorted_commands: Vec<(String, usize)> = command_counts.into_iter().collect();

    sorted_commands.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    println!("--- Your Most Used Commands ---");
    for (i, (cmd, count)) in sorted_commands.iter().take(3).enumerate() {
        println!("{}. \"{}\" ({} uses)", i + 1, cmd, count);
    }
}


