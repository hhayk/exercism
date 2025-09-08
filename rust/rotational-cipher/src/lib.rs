use std::{char, u8};

pub fn rotate(input: &str, key: u8) -> String {
    //    todo!(
    //        "How would input text '{input}' transform when every letter is shifted using key '{key}'?"
    //    );
    input
        .chars()
        .map(|ch| {
            if ch.is_alphabetic() {
                let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };

                let ch_normal = (ch as u8) - base;
                let u = base + (ch_normal + key) % 26;

                u as char
            } else {
                ch
            }
        })
        .collect()
}
