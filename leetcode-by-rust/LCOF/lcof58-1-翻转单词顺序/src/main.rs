struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s_list: Vec<String> = s.split_ascii_whitespace().map(|x| x.trim().to_string()).collect();
        let mut result = "".to_string();
        // println!("{:?}", s_list);
        for word in s_list {
            result = format!("{} {}", word, result);
        }

        result.trim().to_string()
    }
}

fn main() {
    let s = "the   sky is blue".to_string();
    // println!("{}", Solution::reverse_words(s));
    assert_eq!(Solution::reverse_words(s), "blue is sky the".to_string());
    let s = "  hello world!  ".to_string();
    assert_eq!(Solution::reverse_words(s), "world! hello".to_string());
}
