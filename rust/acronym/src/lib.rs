pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");
    //
    phrase
        .split(' ')
        .flat_map(|str| str.split('-'))
        .map(abbr_word)
        .collect::<Vec<String>>()
        .join("")
}

fn abbr_word(word: &str) -> String {
    let mut ret = Vec::new();
    let mut ignore_next_boolean = false;

    for ch in word.chars() {
        if ch.is_uppercase() {
            if !ignore_next_boolean {
                ret.push(ch);
                ignore_next_boolean = true;
            }
        } else {
            ignore_next_boolean = false;
        }
    }

    if ret.is_empty() {
        if let Some(ch) = word.chars().next() {
            ret.push(ch.to_ascii_uppercase());
        };
    }

    ret.iter().collect()
}
