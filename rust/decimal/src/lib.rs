use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Decimal(BigRational);

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let parts: Vec<&str> = input.split('.').collect();
        let frac = parts.get(1).unwrap_or(&"");
        let num = (parts[0].to_string() + frac).parse::<BigInt>().ok()?;
        let den = 10.to_bigint()?.pow(frac.len() as u32);
        Some(Self(BigRational::new(num, den).reduced()))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
