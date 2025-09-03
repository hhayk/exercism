/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    let mut vec = [false; 26];
    let sentence = sentence.chars().filter(|ch| ch.is_alphabetic());

    for ch in sentence {
        let char_code = ch.to_ascii_lowercase() as u32;
        let index = (char_code - ('a' as u32)) as usize;
        vec[index] = true;
    }

    vec.iter().all(|&b| b)
}
