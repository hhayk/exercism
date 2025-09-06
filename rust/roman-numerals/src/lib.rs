use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    arabic_num: u32,
}

const ROMAN_MAP: &[(u32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        // todo!("Return a roman-numeral string representation of the Roman object");
        let mut num = self.arabic_num;
        let mut result = String::new();

        for &(value, symbol) in ROMAN_MAP {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }

        write!(_f, "{}", result)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        // todo!("Construct a Roman object from the '{num}' number");
        if num == 0 || num > 3999 {
            panic!("Number must be between 1 and 3999 for traditional Roman numerals.");
        }
        Roman { arabic_num: num }
    }
}
