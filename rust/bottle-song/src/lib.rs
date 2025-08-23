pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    fn number_to_words(n: u32) -> String {
        let numbers = [
            "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        ];
        numbers.get(n as usize).unwrap().to_string()
    }
    fn bottle_word(n: u32) -> String {
        if n == 1 {
            "bottle".to_string()
        } else {
            "bottles".to_string()
        }
    }
    let mut verses = Vec::new();

    for i in (start_bottles - take_down + 1..=start_bottles).rev() {
        let verse = [
            format!(
                "{} green {} hanging on the wall,",
                capitalize_first_letter(&number_to_words(i)),
                bottle_word(i),
            ),
            format!(
                "{} green {} hanging on the wall,",
                capitalize_first_letter(&number_to_words(i)),
                bottle_word(i),
            ),
            "And if one green bottle should accidentally fall,".to_string(),
            format!(
                "There'll be {} green {} hanging on the wall.",
                number_to_words(i - 1),
                bottle_word(i - 1),
            ),
        ]
        .join("\n");
        verses.push(verse);
    }

    verses.join("\n\n")
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
