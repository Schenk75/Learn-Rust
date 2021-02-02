use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<&i32> = HashSet::new();
        for num in nums.iter() {
            if set.contains(num) {
                return *num;
            }
            set.insert(num);
        }
        -1
    }
}



fn main() {
    let nums = vec![2, 3, 1, 0, 2, 5, 3];
    println!("{}", Solution::find_repeat_number(nums));
}