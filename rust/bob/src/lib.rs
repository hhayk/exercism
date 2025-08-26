pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")

    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if message.ends_with('?') {
        if message
            .chars()
            .all(|c| c.is_uppercase() || c == '?' || c == '\'' || c == ' ')
        {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }

    let alphabetics: Vec<char> = message.chars().filter(|c| c.is_alphabetic()).collect();

    if !alphabetics.is_empty() && alphabetics.iter().all(|c| c.is_uppercase()) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
