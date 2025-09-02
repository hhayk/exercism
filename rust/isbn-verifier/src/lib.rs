/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let has_invalid_chars = isbn.chars().any(|ch| ch.is_alphabetic() && ch != 'X');
    if has_invalid_chars {
        return false;
    }

    let pure_chars: Vec<char> = isbn
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == 'X')
        .collect();

    if pure_chars.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, &c) in pure_chars.iter().enumerate() {
        let value = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'X' if i == 9 => 10,
            _ => return false,
        };
        sum += value * (10 - i as u32);
    }
    sum % 11 == 0
}
