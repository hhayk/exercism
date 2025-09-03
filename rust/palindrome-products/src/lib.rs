use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        //todo!("return the value of the palindrome")
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        // todo!("return the set of factors of the palindrome")
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    //    todo!(
    //        "returns the minimum palindrome and maximum palindrome of the products of two factors in the range {min} to {max}"
    //    );
    let mut map = HashMap::new();
    // This can be optimazied to only calculate min and max with 2 iterations.
    for a in min..=max {
        for b in a..=max {
            let key = a * b;
            if is_palindrome(key) {
                map.entry(key).or_insert(HashSet::new()).insert((a, b));
            }
        }
    }

    match (map.keys().min(), map.keys().max()) {
        (Some(min), Some(max)) => Some((
            (Palindrome {
                value: *min,
                factors: map.get(min).cloned().unwrap(),
            }),
            (Palindrome {
                value: *max,
                factors: map.get(max).cloned().unwrap(),
            }),
        )),
        _ => None,
    }
}

fn is_palindrome(num: u64) -> bool {
    let mut num = num;
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    let mut l = 0;
    let mut r = digits.len() - 1;

    while l < r {
        if digits[l] != digits[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}
