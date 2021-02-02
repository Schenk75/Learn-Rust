use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = nums[0];
        let mut max_num = dp;
        for i in 1..nums.len() {
            dp = max(nums[i], nums[i]+dp);
            if dp > max_num {max_num = dp;}
        }
        max_num
    }
}

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("{}", Solution::max_sub_array(nums));
}
