#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    //todo!("Convert the values {values:?} to a list of bytes")
    let mut ans = Vec::new();
    let mask = 0x7F;

    for &value in values {
        let mut tmp = vec![(value & mask) as u8];

        let mut value = value >> 7;
        while value > 0 {
            tmp.push((value & mask) as u8 + 0x80);
            value >>= 7;
        }

        tmp.reverse();
        ans.append(&mut tmp);
    }

    ans
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    //todo!("Convert the list of bytes {bytes:?} to a list of numbers")
    let mut ans = Vec::new();
    let mut tmp = 0;
    let mask = 0x7F;

    for &byte in bytes.iter() {
        tmp <<= 7;
        if byte & 0x80 == 0 {
            tmp += u32::from(byte);
            ans.push(tmp);
            tmp = 0;
        } else {
            tmp += u32::from(byte & mask);
        }
    }

    if ans.is_empty() {
        Err(Error::IncompleteNumber)
    } else {
        Ok(ans)
    }
}
