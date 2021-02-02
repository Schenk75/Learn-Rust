use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {return 0;}
        let mut tmp = prices[1] - prices[0];
        let mut max_p = max(tmp, 0);
        let mut dp;
        for i in 2..prices.len() {
            dp = max(prices[i] - prices[i-1], tmp + prices[i] - prices[i-1]);
            if dp > max_p {max_p = dp;}
            tmp = dp;
        }

        max_p
    }
}

fn main() {
    let prices = vec![1,23];
    println!("{}", Solution::max_profit(prices));
}
