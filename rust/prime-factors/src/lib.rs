pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut d = 2;
    let mut res = Vec::new();

    while n > 1 {
        while n % d == 0 {
            res.push(d);
            n /= d;
        }
        d += 1
    }

    res
}
