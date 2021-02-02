use std::cmp::max;

struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        let mut dp = vec![0; (n+1) as usize];
        dp[1] = 0;
        dp[2] = 1;

        for i in 3..=n {
            for j in 0..i {
                dp[i as usize] = max(dp[i as usize], max(dp[(i-j) as usize] * j, j * (i-j)));
            }
        }
        
        dp[(n) as usize]
    }
}

fn main() {
    let n = 30;
    println!("{}", Solution::cutting_rope(n));
}
