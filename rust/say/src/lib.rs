pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let groups = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    let mut parts = Vec::new();
    let mut num = n;
    let mut group_idx = 0;

    while num > 0 {
        let chunk = num % 1000;
        if chunk > 0 {
            let mut chunk_parts = Vec::new();

            // Hundreds
            let hundreds = chunk / 100;
            if hundreds > 0 {
                chunk_parts.push(format!("{} hundred", units[hundreds as usize]));
            }

            // Tens and units
            let remainder = chunk % 100;
            if remainder >= 20 {
                let ten_digit = remainder / 10;
                let unit_digit = remainder % 10;
                if unit_digit > 0 {
                    chunk_parts.push(format!(
                        "{}-{}",
                        tens[ten_digit as usize], units[unit_digit as usize]
                    ));
                } else {
                    chunk_parts.push(tens[ten_digit as usize].to_string());
                }
            } else if remainder >= 10 {
                chunk_parts.push(teens[(remainder - 10) as usize].to_string());
            } else if remainder > 0 {
                chunk_parts.push(units[remainder as usize].to_string());
            }

            let mut formatted_chunk = chunk_parts.join(" ");

            if group_idx > 0 {
                formatted_chunk = format!("{} {}", formatted_chunk, groups[group_idx]);
            }

            parts.push(formatted_chunk);
        }

        num /= 1000;
        group_idx += 1;
    }

    parts.iter().rev().cloned().collect::<Vec<_>>().join(" ")
}
