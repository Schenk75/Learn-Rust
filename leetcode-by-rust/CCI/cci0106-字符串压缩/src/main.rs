struct Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        let mut res = String::new();
        let len = s.len();
        let mut tmp = ' ';
        let mut cnt = 0;

        for ch in s.chars() {
            if ch != tmp {
                tmp = ch;
                if cnt != 0 {
                    res.push_str(&cnt.to_string());
                    cnt = 0;
                }
                res.push(tmp);
                if res.len() >= len {return s;}
            }
            cnt += 1;
        }
        if cnt != 0 {
            res.push_str(&cnt.to_string());
            if res.len() >= len {return s;}
        }
    
        res
    }
}

fn main() {
    let s = "aabcccccaaa".to_string();
    println!("{}", Solution::compress_string(s));
}
