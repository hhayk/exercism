pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    //todo!("find the saddle points of the following matrix: {input:?}")
    //
    let mut ans: Vec<(usize, usize)> = vec![];

    for (row_idx, rows) in input.iter().enumerate() {
        if let Some(row_max) = rows.iter().max() {
            for (col_idx, el) in rows.iter().enumerate() {
                if el == row_max {
                    let mut cols = vec![];
                    for rows2 in input.iter() {
                        cols.push(rows2[col_idx]);
                    }

                    if let Some(col_min) = cols.iter().min() {
                        if el == col_min {
                            ans.push((row_idx, col_idx));
                        }
                    }
                }
            }
        }
    }

    ans
}
