// I don't know chess and I couldn't know how to calculate it. 🤷‍♂️
pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s < 65, "Square must be between 1 and 64");
    1 << (s - 1)
}

pub fn total() -> u64 {
    -1i64 as u64
}
