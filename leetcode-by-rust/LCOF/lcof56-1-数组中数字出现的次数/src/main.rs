struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        // 获得数组所有元素的异或，也等于两个只出现一次的数的异或
        let mut xor = 0;
        for num in nums.iter() {
           xor = xor ^ num;
        }

        //获得xor中最低位的1，可作为区分两个只出现一次的数的mask
        let mut mask = 1;
        while mask & xor == 0 {
            mask <<= 1;
        }

        // 根据mask分类数组的数，分两组进行异或，得到的两个数就是只出现一次的数
        let (mut xor_1, mut xor_0) = (0, 0);
        for num in nums.iter() {
            if num & mask == 0 {
                xor_0 ^= num;
            } else {
                xor_1 ^= num;
            }
        }

        vec![xor_0, xor_1]
    }
}

fn main() {
    let nums = vec![1,2,10,4,1,4,3,3];
    println!("{:?}", Solution::single_numbers(nums));
}
