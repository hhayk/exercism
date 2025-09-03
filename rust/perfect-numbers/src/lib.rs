#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    // todo!("classify {num}");
    //

    if num == 0 {
        return None;
    }

    let mut sum = 0;

    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }

    let res = if sum < num {
        Classification::Deficient
    } else if sum > num {
        Classification::Abundant
    } else {
        Classification::Perfect
    };

    Some(res)
}
