pub fn encode(source: &str) -> String {
    // todo!("Return the run-length encoding of {source}.");
    let mut encoded_string = String::new();
    let mut chars = source.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count = 1;

        while let Some(&next_char) = chars.peek() {
            if next_char == current_char {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }

        if count > 1 {
            encoded_string.push_str(&count.to_string());
        }
        encoded_string.push(current_char);
    }

    encoded_string
}

pub fn decode(source: &str) -> String {
    // todo!("Return the run-length decoding of {source}.");
    let mut decoded_string = String::new();
    let mut count_str = String::new();

    for ch in source.chars() {
        if ch.is_numeric() {
            count_str.push(ch);
        } else {
            let count = count_str.parse().ok().unwrap_or(1);
            count_str.clear();

            let ss = String::from_iter(vec![&ch; count]);
            decoded_string.push_str(&ss);
        }
    }

    decoded_string
}
