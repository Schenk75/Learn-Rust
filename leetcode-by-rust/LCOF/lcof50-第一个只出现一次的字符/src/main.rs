use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut map: HashMap<char, bool> = HashMap::new();
        let mut ch_set = vec![];

        for ch in s.chars() {
            if let Some(_idx) = map.get(&ch) {
                map.insert(ch, false);
            } else {
                map.insert(ch, true);
                ch_set.push(ch);
            }
        }

        for ch in ch_set {
            if *map.get(&ch).unwrap() {return ch;}
        }
        ' '
    }
}

fn main() {
    let s = "abaccdeff".to_string();
    println!("{}", Solution::first_uniq_char(s));
}
