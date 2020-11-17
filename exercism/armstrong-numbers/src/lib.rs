pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_mut = num;
    let mut digits: Vec<u8> = Vec::new();

    while num_mut != 0 {
        digits.push((num_mut % 10) as u8);
        num_mut /= 10;
    }

    let mut result: u32 = 0;
    let len: u32 = digits.len() as u32;
    
    for &i in digits.iter() {
        result += (i as u32).pow(len);
    }

    num == result
}
