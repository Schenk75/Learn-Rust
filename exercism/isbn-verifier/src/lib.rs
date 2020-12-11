/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut count_down = 10;

    for ch in isbn.chars() {
        if count_down == 0 { return false; }

        let num = ch.to_string().parse::<u32>();
        match num {
            Ok(n) => {
                sum += n * count_down;
            },
            Err(_) => {
                if ch == 'X' {
                    if count_down != 1 { return false; }
                    sum += count_down * 10;
                } else if ch == '-' {
                    continue;
                } else {
                    return false;
                }
            },
        }
        
        count_down -= 1;
    }

    if count_down != 0 { return false; }
    // println!("{}", sum);

    sum % 11 == 0
}
