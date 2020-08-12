pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];

    for num in 3.. {
        if primes.len() as u32 > n {
            break;
        }
        if primes.iter().any(|&prime| num % prime == 0) {
            continue;
        } else {
            primes.push(num);
        }
    }

    *primes.last().unwrap()
}
