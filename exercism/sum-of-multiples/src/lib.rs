pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit == 0 {
        return 0;
    }
    let mut list = vec![0];

    for &factor in factors.iter() {
        if factor == 0 {
            continue;
        }
        
        for num in (factor..limit).step_by(factor as usize) {
            list.push(num);
        }
    }
    list.sort();
    list.dedup();
    list.iter().sum()
}