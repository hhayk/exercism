use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    // todo!("Score {word} in Scrabble.");
    let scores = HashMap::from([
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);

    word.chars()
        .flat_map(|ch| {
            scores.iter().find_map(|(&p, v)| {
                if v.contains(&ch.to_ascii_uppercase()) {
                    Some(p)
                } else {
                    None
                }
            })
        })
        .sum()
}
