/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // todo!("Encoding of {plain:?} in Atbash cipher.");
    plain
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .map(shift)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|iter| iter.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    // todo!("Decoding of {cipher:?} in Atbash cipher.");
    //
    cipher
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(shift)
        .collect()
}

fn shift(ch: char) -> char {
    if ch.is_numeric() {
        ch
    } else {
        let offset = ch as u8 - b'a';
        (b'z' - offset) as char
    }
}
