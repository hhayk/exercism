pub struct Matrix {
    // Implement your Matrix struct
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        //todo!("Create new method to store the {input}")
        let data = input
            .split("\n")
            .map(|row| {
                row.split(" ")
                    .map(|digit| digit.parse().unwrap_or(0))
                    .collect::<Vec<u32>>()
            })
            .collect();

        Self { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        // todo!("Return the row at {row_no} (1-indexed) or None if the number is invalid")
        if let Some(col) = self.data.first().map(|row| row.len()) {
            let range = (row_no - 1) * col..row_no * col;

            self.data
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<u32>>()
                .get(range)
                .map(|v| v.to_vec())
        } else {
            None
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        // todo!("Return the column at {col_no} (1-indexed) or None if the number is invalid")
        if let Some(col) = self.data.first().map(|row| row.len()) {
            let res: Vec<u32> = self
                .data
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<u32>>()
                .iter()
                .enumerate()
                .filter_map(|(index, &el)| {
                    if index % col == col_no - 1 {
                        Some(el)
                    } else {
                        None
                    }
                })
                .collect();

            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        } else {
            None
        }
    }
}
