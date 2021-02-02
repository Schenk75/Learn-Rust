struct Solution;

impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {return nums;}
        let (mut i, mut j) = (0, nums.len()-1);
        while i < j {
            if nums[i] % 2 == 0 && nums[j] % 2 == 1 {
                let tmp = nums[i];
                nums[i] = nums[j];
                nums[j] = tmp;
                i += 1;
                j -= 1;
            } else if nums[i] % 2 == 0 {
                j -= 1;
            } else if nums[j] % 2 == 1 {
                i += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        nums
    }
}

fn main() {
    let nums = vec![1,2,3,4];
    println!("{:?}", Solution::exchange(nums));
}
