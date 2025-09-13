#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    if let Some(ch) = string_digits.chars().find(|ch| !ch.is_ascii_digit()) {
        return Err(Error::InvalidDigit(ch));
    }

    let digits = string_digits
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let max_product = digits
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .unwrap_or(0);

    Ok(max_product)
}
