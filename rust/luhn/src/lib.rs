/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    let trimmed_code = code.trim();

    if trimmed_code.len() <= 1 {
        return false;
    }

    let digits: Option<Vec<u32>> = trimmed_code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect();

    let digits = match digits {
        Some(d) if d.len() > 1 => d,
        _ => return false,
    };

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &e)| {
            if i % 2 == 1 {
                let d = e * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                e
            }
        })
        .sum();

    sum % 10 == 0
}
