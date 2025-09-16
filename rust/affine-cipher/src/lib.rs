/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // todo!("Encode {plaintext} with the key ({a}, {b})");
    //
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let encoded_chars: Vec<char> = plaintext
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let x = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                let encoded_val = (a * x + b) % M;
                Some(((b'a' as i32 + encoded_val) as u8) as char)
            } else if c.is_ascii_digit() {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    // 2. Group the result into chunks of 5, separated by spaces.
    let result = encoded_chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // todo!("Decode {ciphertext} with the key ({a}, {b})");
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mmi = (1..M).find(|n| (a * n) % M == 1).unwrap();

    let decoded_string: String = ciphertext
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let y = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                let decoded_val = (mmi * (y - b)).rem_euclid(M);
                Some(((b'a' as i32 + decoded_val) as u8) as char)
            } else if c.is_ascii_digit() {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    Ok(decoded_string)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
