pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors = factors.iter().filter(|f| **f != 0).collect::<Vec<&u32>>();
    (1..limit)
        .filter(|n| factors.iter().any(|f| n % **f == 0))
        .sum()
}
