struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut binary = vec![0; 32];
        // 统计每个数的二进制表示中每一位的1的个数
        for num in nums {
            let mut n = num;
            for i in 0..32 {
                binary[i] += n & 1;
                n >>= 1;
            }
        }

        // println!("{:?}", binary);
        // 如果某一位的1的个数不能被3整除，说明目标数字的该位为1
        let mut result = 0;
        for (i, b) in binary.iter().enumerate() {
            if b % 3 != 0 {
                result += 2i32.pow(i as u32);
            }
        }

        result
    }

    // 有限自动状态机【没搞懂】
    pub fn single_number_fsm(nums: Vec<i32>) -> i32 {
        // ab ab ab ab
        // 00 01 11 00
        let mut a = 0;
        let mut b = 0;
        for n in nums {
            let t = a;
            a = (n & !a & b) + (!n & a);
            b = (n & !t) + (!n & b);
            // println!("{:?}", (n, a, b));
        }
        b
    }
}

fn main() {
    let nums = vec![3,4,3,3];
    println!("{}", Solution::single_number(nums));
}
