use std::collections::HashSet;

struct Solution;

impl Solution {
    // 集合
    pub fn is_unique_set(astr: String) -> bool {
        let mut set = HashSet::new();

        for s in astr.chars() {
            if set.contains(&s) {return false;}
            else {set.insert(s);}
        }
        true
    }

    // 位运算(需要提前知道字符集的大小)
    pub fn is_unique(astr: String) -> bool {
        let mut mark: u32 = 0;

        for s in astr.bytes() {
            let move_idx = s - 97;
            if mark & (1 << move_idx) != 0 {return false;}
            mark = mark | (1 << move_idx)
        }

        true
    }
}

fn main() {
    let astr = "leetcode".to_string();
    println!("{}", Solution::is_unique_set(astr.clone()));
    println!("{}", Solution::is_unique(astr.clone()));
}
