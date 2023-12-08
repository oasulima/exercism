pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    (2..=s).fold(1u64, |sum, _| sum * 2)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
