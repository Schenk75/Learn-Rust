pub fn factors(n: u64) -> Vec<u64> {
    if is_prime(n) { return vec![n]; }
    let mut result: Vec<u64> = Vec::new();
    let mut n = n;
    for i in 2..(n/2 as u64) {
        if is_prime(i) {
            while n != 1 && n % i == 0 {
                n /= i;
                result.push(i);
            }
        }
        if n == 1 { break; }
    }
    result
}


pub fn is_prime(n: u64) -> bool {
    if n == 2 { return true; }

    if n % 2 == 0 { return false; }

    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }

    true
}