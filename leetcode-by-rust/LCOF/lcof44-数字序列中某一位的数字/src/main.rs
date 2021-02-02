struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let n = n as i64;
        let mut count = 0;
        let mut digit = 1;   // 位数
        
        loop {
            let nums_cnt = 9 * 10i64.pow(digit-1);      // 位数对应的数字个数=9*10^(位数-1)
            let digit_cnt = nums_cnt * (digit as i64);     // 位数对应的数位=位数*数字个数

            if n > count + digit_cnt {
                count += digit_cnt;
                digit += 1;
                continue;
            }

            let mut start_num = 10i64.pow(digit-1);
            let digits_to_count = n - count;        // 从start_num开始还要往后数的数位
            let nums_to_count = digits_to_count / digit as i64;
            start_num += nums_to_count - 1;
            let remain = digits_to_count % digit as i64;
            if remain != 0 {
                start_num += 1;
                for _i in 0..(digit as i64-remain) {
                    start_num /= 10;
                }
            }
            return (start_num % 10) as i32;
        }
    }
}

fn main() {
    let n = 10;
    println!("{}", Solution::find_nth_digit(n));
}
