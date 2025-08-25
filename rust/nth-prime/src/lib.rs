pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let limit = 10_000_000;
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes_found = -1;

    for p in 2..limit {
        if is_prime[p] {
            primes_found += 1;
            if primes_found == n as i32 {
                return p as u32;
            }

            for i in (p * p..limit).step_by(p) {
                is_prime[i] = false;
            }
        }
    }

    panic!("Prime number not found within limit");
}
