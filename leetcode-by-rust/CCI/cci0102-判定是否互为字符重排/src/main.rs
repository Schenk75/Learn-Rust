use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {return false;}

        let mut map = HashMap::new();

        for s in s1.chars() {
            let count = map.entry(s).or_insert(0);
            *count += 1;
        }

        for s in s2.chars() {
            let count = map.entry(s).or_insert(0);
            *count -= 1;
        }

        for val in map.values() {
            if *val != 0 {return false;}
        }

        true
    }
}

fn main() {
    let (s1, s2) = ("abcc".to_string(), "bca".to_string());
    println!("{}", Solution::check_permutation(s1, s2));
}
