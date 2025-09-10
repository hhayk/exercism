use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //    todo!(
    //        "Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c"
    //    );
    let mut ret = HashSet::new();

    for a in 1..sum / 3 {
        for b in (a + 1)..sum / 2 {
            let c = sum - a - b;

            if c > b {
                if a * a + b * b == c * c {
                    ret.insert([a, b, c]);
                }
            }
        }
    }

    ret
}
