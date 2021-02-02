struct Solution;

impl Solution {
    pub fn str_to_int(str: String) -> i32 {
        let str: Vec<char> = str.trim().chars().collect();
        if str.is_empty() {return 0;}
        let negative;
        let mut result = String::new();
        let mut i = 1;
        if str[0] == '-' {
            negative = true;
            while i < str.len() && str[i] == '0' {i += 1;}
        } else if str[0].is_ascii_digit() {
            negative = false;
            if str[0] != '0' {
                result.push(str[0]);
            } else {
                while i < str.len() && str[i] == '0' {i += 1;}
            }
        } else if str[0] == '+' {
            negative = false;
            while i < str.len() && str[i] == '0' {i += 1;}
        } else {
            return 0;
        }

        while i < str.len() {
            if str[i].is_ascii_digit() {
                result.push(str[i]);
            } else {
                break;
            }
            i += 1;
        }

        if !result.is_empty() {
            if result.len() > 10 || (result.len() == 10 && result > "2147483647".to_string()) {
                return if negative {-2147483648} else {2147483647};
            }
            let result: i32= result.parse().unwrap();
            if negative {
                return -result;
            } else {
                return result;
            }
        } else {
            return 0;
        }
    }
}

fn main() {
    let str = "  0000000000012345678".to_string();
    println!("{}", Solution::str_to_int(str));
}
