use std::collections::HashMap;

struct Solution;

impl Solution {
    // 暴力求解
    pub fn length_of_longest_substring_brute(s: String) -> i32 {
        let mut s_list = Vec::new();
        let mut result = 0;
        for ch in s.chars() {
            if s_list.contains(&ch) {
                let len = s_list.len();
                if len > result {result = len};
                for i in 1..len+1 {
                    if s_list[len-i] == ch {
                        s_list = s_list[len-i+1..].to_vec();

                        break;
                    }
                }
            } 
            s_list.push(ch);
            println!("ch:{} s_list:{:?} result{}", ch, s_list, result);
        }

        if s_list.len() > result {s_list.len() as i32} else {result as i32}
    }

    // 动态规划+哈希表
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut result = 0;
        let mut tmp = 0;

        for (j, ch) in s.chars().enumerate() {
            if let Some(&i) = map.get(&ch) {
                if tmp < j - i {
                    tmp += 1;
                } else {
                    tmp = j - i;
                }
            } else {
                tmp += 1;
            }
            map.insert(ch, j);
            if result < tmp {result = tmp;}
        }

        result as i32
    }
}

fn main() {
    let s = "dvdf".to_string();
    println!("{}", Solution::length_of_longest_substring_brute(s.clone()));
    println!("{}", Solution::length_of_longest_substring(s.clone()));
}
