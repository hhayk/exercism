pub fn annotate(garden: &[&str]) -> Vec<String> {
    //    todo!(
    //        "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    //    );
    //
    if garden.is_empty() {
        return Vec::new();
    }

    let mut ret = Vec::new();

    let (rows, cols) = (garden.len() as i32, garden.first().unwrap().len() as i32);
    for row in 0..rows {
        let mut ss = String::new();
        for col in 0..cols {
            if let Some(ch) = garden[row as usize].chars().nth(col as usize) {
                if ch == '*' {
                    ss.push('*');
                } else {
                    let mut count = 0;
                    for rr in (row - 1)..=(row + 1) {
                        for cc in (col - 1)..=(col + 1) {
                            if rr == row && cc == col {
                                continue;
                            }
                            if let Some(&ri) = garden.get(rr as usize) {
                                if let Some(ch) = ri.chars().nth(cc as usize) {
                                    if ch == '*' {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                    if count == 0 {
                        ss.push(' ');
                    } else {
                        ss.push(char::from_digit(count, 10).unwrap());
                    }
                }
            }
        }
        ret.push(ss);
    }

    ret
}
