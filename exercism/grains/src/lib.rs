pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    if s == 1 {
        return 1;
    }

    let mut result = 1;
    for _ in 1..s {
        result *= 2;
    }
    result
}

pub fn total() -> u64 {
    (1..65).map(|x| square(x)).sum()
}
