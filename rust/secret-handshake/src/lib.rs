pub fn actions(n: u8) -> Vec<&'static str> {
    //todo!("What is the secret handshake for {n}?")
    let mut n = n;
    let mut vec = vec![];
    let mut step = 0;

    while n > 0 {
        if n & 1 == 1 {
            match step {
                0 => vec.push("wink"),
                1 => vec.push("double blink"),
                2 => vec.push("close your eyes"),
                3 => vec.push("jump"),
                4 => vec.reverse(),
                _ => panic!("Out of range"),
            }
        }

        n >>= 1;
        step += 1;
        step %= 5;
    }

    vec
}
