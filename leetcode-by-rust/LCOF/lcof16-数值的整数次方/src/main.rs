struct Solution;

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {return 1.0;}
        let mut result = 1.0;
        let mut n = n as i64;
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        while n > 0 {
            if n % 2 == 1 {
                result *= x;
            }
            x *= x;
            n /= 2;
        }

        result
    }
}

fn main() {
    let (x, n) = (2.0, 2);
    println!("{}", Solution::my_pow(x, n));
}
