use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // todo!("Is {candidate} an isogram?");
    let chars: Vec<char> = candidate
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    let set: HashSet<char> = chars.iter().cloned().collect();

    chars.len() == set.len()
}
