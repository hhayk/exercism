pub fn square(s: u32) -> u64 {
    //todo!("grains of rice on square {s}");
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

pub fn total() -> u64 {
    // todo!();
    (1..=64).map(square).sum()
}
