pub fn square_of_sum(num: usize) -> usize {
    (1..num + 1).sum::<usize>().pow(2)
}

pub fn sum_of_squares(num: usize) -> usize {
    (1..num + 1).map(|n| n * n).sum()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
