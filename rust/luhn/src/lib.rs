/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    if code.trim().len() <= 1 {
        return false;
    }

    if code
        .find(|c: char| !(c.is_ascii_digit() || c == ' '))
        .is_some()
    {
        return false;
    }

    let sum = code
        .split_whitespace()
        .flat_map(|iter| {
            iter.chars()
                .map(|c: char| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .rev()
        .enumerate()
        .map(|(i, e)| {
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
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    sum % 10 == 0
}
