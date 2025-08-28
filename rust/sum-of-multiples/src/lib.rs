pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|f| {
            let step = *f as usize;
            if step >= 1 {
                (*f..limit).step_by(step).collect()
            } else {
                Vec::new()
            }
        })
        .collect::<std::collections::HashSet<_>>()
        .iter()
        .sum()
}
