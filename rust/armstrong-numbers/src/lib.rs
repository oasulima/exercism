pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_rest = num;
    let mut digits: Vec<u8> = Vec::new();

    while num_rest > 0 {
        digits.push((num_rest % 10) as u8);
        num_rest /= 10;
    }

    let power: u32 = digits.len() as u32;
    num as u64 == digits.iter().map(|x| (*x as u64).pow(power)).sum()
}
