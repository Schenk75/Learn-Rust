struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 二分法
        let (mut left, mut right) = (0, (nums.len()-1) as i32);

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] > mid {right = mid - 1;}
            else {left = mid + 1};
        }

        // println!("left:{} right:{}", left, right);

        // if right < 0 {nums[left as usize] - 1} else {nums[right as usize] + 1}
        left
    }
}

fn main() {
    let nums = vec![1,2];
    println!("{}", Solution::missing_number(nums));
}
