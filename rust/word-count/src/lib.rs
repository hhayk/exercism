use std::collections::HashMap;

static APOSTROPHE: char = 39 as char;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    //todo!("Count of occurrences of words in {words:?}");

    let mut ret = HashMap::new();
    let mut word = String::new();

    for ch in words.chars() {
        if ch.is_ascii_alphanumeric() || ch == APOSTROPHE {
            word.push(ch);
        } else {
            append_to_hash(&word, &mut ret);
            word.clear();
        }
    }

    append_to_hash(&word, &mut ret);

    ret
}

fn append_to_hash(word: &str, map: &mut HashMap<String, u32>) {
    let mut word = word.to_string().to_ascii_lowercase();
    if word.starts_with(APOSTROPHE) {
        word.remove(0);
    }
    if word.ends_with(APOSTROPHE) {
        word.pop();
    }
    if !word.is_empty() {
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }
}
