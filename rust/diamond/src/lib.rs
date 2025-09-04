pub fn get_diamond(c: char) -> Vec<String> {
    // todo!("Return the vector of strings which represent the diamond with particular char {c}");
    //
    let n = (c as u8 - b'A') as usize;
    let total_width = 2 * n + 1;
    let mut diamond = Vec::with_capacity(total_width);

    // Top
    for i in 0..=n {
        let current_char = (b'A' + i as u8) as char;
        let outer_spaces = n - i;
        let inner_spaces = if i == 0 { 0 } else { 2 * i - 1 };

        let mut row = String::new();
        row.push_str(&" ".repeat(outer_spaces));
        row.push(current_char);
        if i > 0 {
            row.push_str(&" ".repeat(inner_spaces));
            row.push(current_char);
        }
        row.push_str(&" ".repeat(outer_spaces));

        diamond.push(row);
    }

    // Bottom
    for i in (0..n).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
