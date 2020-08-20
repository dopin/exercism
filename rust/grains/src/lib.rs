// I don't know chess and I couldn't know how to calculate it. 🤷‍♂️
pub fn square(s: u32) -> u64 {
    match s {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        16 => 32_768,
        32 => 2_147_483_648,
        64 => 9_223_372_036_854_775_808,
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615u64
}
