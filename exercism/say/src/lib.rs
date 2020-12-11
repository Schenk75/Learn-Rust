const SCALES: [(u64, &str); 7] = [
    (100, "hundred"),
    (1_000, "thousand"),
    (1_000_000, "million"),
    (1_000_000_000, "billion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000_000_000, "quintillion"),
];
pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        x => {
            if let Some(&(i, s)) = SCALES.iter().rev().find(|(i, _)| x / i != 0) {
                format!("{} {} {}", encode(x / i), s, encode(x % i))
            } else if x / 10 != 0 {
                format!("{}-{}", encode((x / 10) * 10), encode(x % 10))
            } else {
                panic!("Too big!")
            }
        }
    }
    .replace(" zero", "")
}