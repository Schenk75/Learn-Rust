struct Solution;

impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        (n + n.pow(2)) >> 1
    }

    pub fn sum_nums_(n: i32) -> i32 {
        let mut result = 0;
        Self::sum(n, &mut result);
        result
    }

    fn sum(i: i32, res: &mut i32) -> bool {
        i > 1 && Self::sum(i-1, res);
        *res += i;
        true
    }
}

fn main() {
    let n = 10;
    println!("{}", Solution::sum_nums_(n));
}
