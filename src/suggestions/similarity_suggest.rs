use strsim::levenshtein;

pub fn suggest_command<'a>(failed: &'a str, history: &'a [String]) -> Option<&'a String> {
    history
        .iter()
        .filter(|cmd| similarity(failed, cmd) > 0.50)
        .max_by(|a, b| {
            let sim_a = similarity(failed, a);
            let sim_b = similarity(failed, b);
            sim_a
                .partial_cmp(&sim_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub fn similarity(a: &str, b: &str) -> f64 {
    let dist = levenshtein(a, b) as f64;
    let max_len = a.len().max(b.len()) as f64;
    if max_len == 0.0 {
        1.0
    } else {
        1.0 - (dist / max_len)
    }
}
