struct Solution;

impl Solution {
    pub fn add(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 { // 当进位为 0 时跳出
            let c = (a & b) << 1;  // c = 进位
            a ^= b; // a = 非进位和
            b = c; // b = 进位
        }
        a
    }
}

fn main() {
    let (a, b) = (1, 2);
    println!("{}", Solution::add(a, b));
}
