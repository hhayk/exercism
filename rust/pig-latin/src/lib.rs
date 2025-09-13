const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn translate(input: &str) -> String {
    //todo!("Using the Pig Latin text transformation rules, convert the given input '{input}'");
    input
        .split_ascii_whitespace()
        .map(rules)
        .collect::<Vec<_>>()
        .join(" ")
}

fn rules(input: &str) -> String {
    rule1(input)
        .or(rule3(input))
        .or(rule4(input))
        .or(rule2(input))
        .unwrap_or(input.to_string())
}

fn rule1(s: &str) -> Option<String> {
    if let Some(mut index) = s.find(|ch| VOWELS.contains(&ch)) {
        if index == 0 || s.starts_with("xr") || s.starts_with("yt") {
            index = 0;
            Some(format!("{}{}ay", &s[index..], &s[..index]))
        } else {
            None
        }
    } else {
        None
    }
}

fn rule2(s: &str) -> Option<String> {
    if let Some(index) = s.find(|ch| VOWELS.contains(&ch)) {
        if index > 0 {
            Some(format!("{}{}ay", &s[index..], &s[..index]))
        } else {
            None
        }
    } else {
        None
    }
}

fn rule3(s: &str) -> Option<String> {
    if let Some(index) = s.find(|ch| VOWELS.contains(&ch)) {
        if index > 0 && &s[index - 1..index + 1] == "qu" {
            Some(format!("{}{}ay", &s[index + 1..], &s[..index + 1]))
        } else {
            None
        }
    } else {
        None
    }
}

fn rule4(s: &str) -> Option<String> {
    if let Some(index) = s.find(|ch| VOWELS.contains(&ch)).or(s.find(|ch| ch == 'y')) {
        if index > 0 {
            Some(format!("{}{}ay", &s[index..], &s[..index]))
        } else {
            None
        }
    } else {
        None
    }
}
