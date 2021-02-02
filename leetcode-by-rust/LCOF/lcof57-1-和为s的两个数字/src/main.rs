struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, nums.len()-1);
        while i < j {
            if nums[i] + nums[j] < target {
                i += 1;
            } else if nums[i] + nums[j] > target {
                j -= 1;
            } else {
                return vec![nums[i], nums[j]];
            }
        }

        vec![]
    }
}

fn main() {
    let nums = vec![10,26,30,31,47,60];
    let target = 40;
    println!("{:?}", Solution::two_sum(nums, target));
}
