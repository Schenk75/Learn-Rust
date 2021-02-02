struct Solution;

impl Solution {
    // 正常方法(正向解)超时
    pub fn last_remaining_forward(n: i32, m: i32) -> i32 {
        let m = m as usize;
        let mut m_mut;
        let mut nums: Vec<i32> = (0..n).collect();

        for _i in 0..n-1 {
            m_mut = m % nums.len();
            let mut tmp;
            if m_mut != 0 {
                tmp = nums[..m_mut-1].to_vec();
                nums = nums[m_mut..].to_vec();
            } else {
                tmp = vec![];
                nums = nums[m_mut..nums.len()-1].to_vec();
            }
            nums.append(&mut tmp);
        }

        nums[0]
    }

    // 逆向推
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        // 最终留下的数字的初始位置
        let mut pos = 0;

        for i in 2..=n {
            pos = (pos + m) % i;
        }

        pos
    }
}

fn main() {
    let (n, m) = (5, 3);
    println!("{}", Solution::last_remaining_forward(n, m));
    println!("{}", Solution::last_remaining(n, m));
}
