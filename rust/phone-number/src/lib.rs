pub fn number(user_number: &str) -> Option<String> {
    //todo!(
    //"Given the number entered by user '{user_number}', convert it into SMS-friendly format. If the entered number is not a valid NANP number, return None."
    //);
    let digits = user_number
        .chars()
        .filter(|ch| ch.is_numeric())
        .collect::<String>();

    match digits.len() {
        10 => validate(&digits),
        11 if digits.starts_with('1') => validate(&digits[1..]),
        _ => None,
    }
}

fn validate(phone_number: &str) -> Option<String> {
    let filtered_digits: Vec<char> = phone_number
        .chars()
        .enumerate()
        .filter_map(|(index, ch)| {
            if index == 0 || index == 3 {
                if ch.to_digit(10).unwrap_or(0) > 1 {
                    Some(ch)
                } else {
                    None
                }
            } else {
                Some(ch)
            }
        })
        .collect();

    if filtered_digits.len() == phone_number.len() {
        Some(phone_number.to_string())
    } else {
        None
    }
}
