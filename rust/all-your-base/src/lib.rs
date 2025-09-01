#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // todo!("Convert {number:?} from base {from_base} to base {to_base}")
    //

    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&n) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(n));
    }

    let number_in_form_base = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, e)| e * from_base.pow(i as u32))
        .sum();

    Ok(convert_to_base_vec(number_in_form_base, to_base))
}

fn convert_to_base_vec(mut num: u32, base: u32) -> Vec<u32> {
    // Special case for converting zero.
    if num == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    while num > 0 {
        // The remainder is the next digit in the new base.
        let remainder = num % base;
        result.push(remainder);
        num /= base;
    }

    // The digits were pushed in reverse order, so we need to reverse the vector.
    result.reverse();
    result
}
