pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut prime = 1;
    
    while count < n {
        prime += 2;
        while !is_prime(prime) {
            prime += 2;
        }

        count += 1;
    }
    println!("{}", prime);

    prime
}

pub fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    
    if n % 2 == 0 {
        return false;
    }

    for i in 2..=((n as f32).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }

    true
}