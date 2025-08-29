use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let sort_string_chars = |s: &str| {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars
    };

    let sorted_word = sort_string_chars(&word.to_lowercase());
    possible_anagrams
        .iter()
        .filter(|&anagram| {
            let anagram = anagram.to_lowercase();

            if anagram == word.to_lowercase() {
                return false;
            }

            sort_string_chars(&anagram) == sorted_word
        })
        .cloned()
        .collect()
}
