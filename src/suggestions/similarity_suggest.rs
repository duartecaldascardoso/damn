use strsim::levenshtein;

// TODO maybe add a way for the user to define this 
static SIMILARITY_THRESHOLD: f64 = 0.1;

pub fn suggest_command<'a>(failed: &'a str, history: &'a [String]) -> Option<&'a String> {
    let mut best: Option<(&String, f64)> = None;

    for cmd in history {
        let sim = similarity(failed, cmd);
        eprintln!("Comparing: '{}' <-> '{}' : {:.4}", failed, cmd, sim);
        if sim >= SIMILARITY_THRESHOLD {
            match best {
                Some((_, best_sim)) if sim > best_sim => best = Some((cmd, sim)),
                None => best = Some((cmd, sim)),
                _ => {}
            }
        }
    }

    if let Some((approved, score)) = &best {
        eprintln!("Approved suggestion: '{}' (score: {:.4})", approved, score);
        Some(approved)
    } else {
        eprintln!("No command in history reached the similarity threshold ({}) .", SIMILARITY_THRESHOLD);
        None
    }
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
