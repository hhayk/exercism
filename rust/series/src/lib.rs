pub fn series(digits: &str, len: usize) -> Vec<String> {
    //todo!("What are the series of length {len} in string {digits:?}")
    let mut digits: i64 = digits.parse().unwrap_or_default();
    let mut digits_arr = Vec::new();

    while digits > 0 {
        let d = digits % 10;
        digits /= 10;
        digits_arr.push(d);
    }
    digits_arr.reverse();

    if len > digits_arr.len() {
        Vec::new()
    } else {
        let mut res = Vec::new();
        for idx in 0..=(digits_arr.len() - len) {
            let ii = &digits_arr[idx..(idx + len)].iter();

            let ss = &ii
                .clone()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("");
            res.push(ss.to_string());
        }

        res
    }
}
