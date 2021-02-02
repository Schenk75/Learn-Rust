struct Solution;

impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut num_str: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        
        num_str.sort_unstable_by(|a, b| {
            format!("{}{}", a, b).cmp(&format!("{}{}", b, a))
        });

        num_str.join("")
    }
}

fn main() {
    let nums = vec![3,30,34,5,9];
    println!("{}", Solution::min_number(nums));
}
