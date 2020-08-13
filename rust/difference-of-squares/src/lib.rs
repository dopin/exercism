pub fn square_of_sum(num: usize) -> usize {
  (1..num+1).fold(0, |acc, n| acc + n).pow(2)
}

pub fn sum_of_squares(num: usize) -> usize {
  (1..num+1).map(|n| n * n).fold(0, |acc, n| acc + n)
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}