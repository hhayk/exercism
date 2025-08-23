pub fn is_armstrong_number(num: u32) -> bool {
    //todo!("true if {num} is an armstrong number")
    let mut digits = Vec::new();
    let mut num_copy = num;
    while num_copy > 0 {
        digits.push(num_copy % 10);
        num_copy /= 10;
    }

    let len = digits.len() as u32;
    let sum = digits.into_iter().map(|d| d.pow(len)).sum::<u32>();

    sum == num
}
