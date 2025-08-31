pub fn annotate(garden: &[&str]) -> Vec<String> {
    //    todo!(
    //        "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    //    );
    //
    if garden.is_empty() {
        return Vec::new();
    }
    if garden[0].is_empty() {
        return vec![String::new()];
    }

    let rows = garden.len();
    let cols = garden[0].len();

    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| {
                    if garden[row].as_bytes()[col] == b'*' {
                        '*'
                    } else {
                        match count_neighbour_flowers(garden, row, col) {
                            0 => ' ',
                            count => char::from_digit(count.into(), 10).unwrap(),
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn count_neighbour_flowers(garden: &[&str], row: usize, col: usize) -> u8 {
    let rows = garden.len() as isize;
    let cols = garden[0].len() as isize;

    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                if garden[new_row as usize].as_bytes()[new_col as usize] == b'*' {
                    count += 1;
                }
            }
        }
    }

    count
}
