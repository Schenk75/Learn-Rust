struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        format!("{}{}", &s[n as usize..], &s[..n as usize])
    }
}

fn main() {
    let s = "abcdefg".to_string();
    let k = 2;
    println!("{}", Solution::reverse_left_words(s, k));
}
