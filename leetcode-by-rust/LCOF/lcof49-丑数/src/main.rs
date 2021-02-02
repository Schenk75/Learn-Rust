use std::cmp::min;

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![1; n as usize];
        let (mut a, mut b, mut c) = (0, 0, 0);

        for i in 1..n as usize {
            let (n2, n3, n5) = (dp[a] * 2, dp[b] * 3, dp[c] * 5);
            dp[i] = min(n2, min(n3, n5));

            if dp[i] == n2 {a += 1;}
            if dp[i] == n3 {b += 1;}
            if dp[i] == n5 {c += 1;}
        }

        *dp.last().unwrap()
    }
}

fn main() {
    let n = 10;
    println!("{}", Solution::nth_ugly_number(n));
}
