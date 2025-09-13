pub fn encrypt(input: &str) -> String {
    //todo!("Encrypt {input:?} using a square code")
    let normalaized_input: Vec<char> = input
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();

    let len = normalaized_input.len();
    let chunks = (len as f32).sqrt().ceil() as usize;

    let rows = normalaized_input
        .chunks(if chunks == 0 { 1 } else { chunks })
        .map(|winodw| winodw.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut vec = vec!["".to_string(); chunks];
    for row in rows {
        for (c, ch) in row.chars().enumerate() {
            if let Some(vec_row) = vec.get_mut(c) {
                vec_row.push(ch);
            }
        }
    }

    let len = vec.first().map(|f| f.len()).unwrap_or(0);
    for s in &mut vec {
        while s.len() < len {
            s.push(' ');
        }
    }

    vec.join(" ")
}
