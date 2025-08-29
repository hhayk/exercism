use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut res = HashSet::new();

    let mut sw: Vec<_> = word[..].to_lowercase().chars().collect();
    sw.sort();

    for pa in possible_anagrams {
        if word.to_lowercase() != *pa.to_lowercase() {
            let mut spa: Vec<_> = pa[..].to_lowercase().chars().collect();
            spa.sort();

            let matching = sw.iter().zip(spa.iter()).filter(|&(a, b)| a == b).count();
            if matching == sw.len() && matching == spa.len() {
                res.insert(*pa);
            }
        }
    }

    res
}
