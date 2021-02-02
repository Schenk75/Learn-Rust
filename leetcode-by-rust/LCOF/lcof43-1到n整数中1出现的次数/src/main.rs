struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n < 1 {return 0;}
        let mut count = 0;
        let mut n_mut = n;
        let mut i = 0;
        let n = n as i64;

        while n_mut != 0 {
            let digit = 10i64.pow(i as u32);
            let curr = n_mut % 10;    // 当前位
            // println!("数字: {}\ndigit: {}\n高位: {}\n低位: {}", curr, digit, n/(digit*10), n % digit);

            // 该位为0 【高位为 n/(digit*10)  低位为 n%digit】
            if curr == 0 {
                count += digit * (n / (digit*10));
            }
            // 该位为1
            else if curr == 1 {
                count += digit * (n / (digit*10)) + 1 + (n % digit);
            }
            // 其他
            else {
                count += digit * (n / (digit*10) + 1);
            }
            // println!("{}: {}", n_list.len() - 1 - i, count)
            i += 1;
            n_mut /= 10;
        }
        count as i32
    }
}

fn main() {
    let n = 12;
    println!("{}", Solution::count_digit_one(n));
}
