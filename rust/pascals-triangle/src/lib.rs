pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // todo!("create Pascal's triangle with {row_count} rows");
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut ret: Vec<Vec<_>> = vec![];

        for idx in 0..self.row_count {
            let mut inner = vec![1; (idx + 1) as usize];
            for idx2 in 1..idx {
                let idx2 = idx2 as usize;
                let last = ret.last().unwrap();
                let v1 = last[idx2 - 1];
                let v2 = last[idx2];
                inner[idx2] = v1 + v2;
            }
            ret.push(inner);
        }

        ret
    }
}
