use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut map = HashMap::new();

        for ch in s.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        let mut odd_flag = if s.len() % 2 == 0 {false} else {true};
        for key in map.keys() {
            if map[key] % 2 == 1 {
                if !odd_flag {return false;}
                else {odd_flag = false;}
            }
        }

        true
    }
}

fn main() {
    let s = "tafctcoa".to_string();
    println!("{}", Solution::can_permute_palindrome(s));
}
