struct Solution;

impl Solution {
    pub fn replace_spaces(s: String, length: i32) -> String {
        let mut res = String::new();

        for (i, ch) in s.chars().enumerate() {
            if i >= length as usize {break;}

            if ch == ' ' {res.push_str("%20");}
            else {res.push(ch);}
        }

        res
    }
}

fn main() {
    let s = "Mr John Smith    ".to_string();
    let length = 13;
    println!("{}", Solution::replace_spaces(s, length));
}
                       