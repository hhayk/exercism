pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        //todo!("Determine if the current Luhn struct contains a valid credit card number.");

        let trimmed_code = self.code.trim();

        if trimmed_code.len() <= 1 {
            return false;
        }

        let digits: Option<Vec<u32>> = trimmed_code
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10))
            .collect();

        let digits = match digits {
            Some(d) if d.len() > 1 => d,
            _ => return false,
        };

        let sum: u32 = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &e)| {
                if i % 2 == 1 {
                    let d = e * 2;
                    if d > 9 {
                        d - 9
                    } else {
                        d
                    }
                } else {
                    e
                }
            })
            .sum();

        sum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        // todo!("From the given input '{input}' create a new Luhn struct.");
        Self {
            code: input.to_string(),
        }
    }
}
