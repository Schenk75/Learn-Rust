struct Solution;

impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();
        let mut zeros = 0;
        let mut cnt = 0;
        // 统计0的个数
        for i in 0..5 {
            if nums[i] == 0 {zeros += 1;}
            else {break;}
        }

        for i in zeros..4 {
            if nums[i] == nums[i+1] {return false;}
            if nums[i+1] - nums[i] < 4 {
                cnt += nums[i+1] - nums[i] -  1;
            } else {return false;}
        }

        if cnt > zeros as i32 {return false;}

        true
    }
}

fn main() {
    let nums = vec![0,0,0,0,0];
    println!("{}", Solution::is_straight(nums));
}
