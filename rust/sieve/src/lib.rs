pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    //todo!("Construct a vector of all primes up to {upper_bound}");
    let upper_bound = upper_bound.try_into().unwrap();
    let mut is_prime = vec![true; upper_bound + 1];

    if upper_bound < 2 {
        return vec![];
    }

    is_prime[0] = false;
    is_prime[1] = false;

    for idx in 2..=(upper_bound as f64).sqrt() as usize {
        if is_prime[idx] {
            for idx2 in (idx * idx..=upper_bound).step_by(idx) {
                is_prime[idx2] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(idx, &state)| if state { Some(idx as u64) } else { None })
        .collect()
}
