pub fn brackets_are_balanced(string: &str) -> bool {
    let open = ['[', '{', '('];
    let close = [']', '}', ')'];
    let mut stack = vec![];

    for ch in string.chars() {
        if open.contains(&ch) {
            stack.push(ch);
        } else if close.contains(&ch) {
            if let Some(&el) = stack.iter().last() {
                if open.iter().position(|&c| c == el) == close.iter().position(|&c| c == ch) {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
