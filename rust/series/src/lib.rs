pub fn series(digits: &str, len: usize) -> Vec<String> {
    //todo!("What are the series of length {len} in string {digits:?}")
    if len > digits.len() {
        return Vec::new();
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|w| String::from_utf8(w.to_vec()).unwrap())
        .collect()
}
