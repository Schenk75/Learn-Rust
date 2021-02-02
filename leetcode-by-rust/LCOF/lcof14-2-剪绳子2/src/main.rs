struct Solution;

impl Solution {
    pub fn cutting_rope(mut n: u32) -> u32 {
        if n < 4 {return n-1;}
        if n == 4 {return n;}

        let mut result = 1;
        while n > 4 {
            result = (result * 3) % 1000000007;
            n -= 3;
        }
        (result * n) % 1000000007
    }
}

fn main() {
    let n = 120;
    println!("{}", Solution::cutting_rope(n));
}
