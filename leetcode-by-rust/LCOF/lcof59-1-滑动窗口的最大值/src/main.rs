use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {return nums;}
        let k = k as usize;
        let (mut i, mut j) = (1, k);
        let mut result = vec![];
        let mut deque = VecDeque::with_capacity(k);

        for m in 0..k {
            Self::push_element(nums[m], &mut deque);
        }
        result.push(*deque.front().unwrap());

        while j < nums.len() {
            // 如果nums[i]是上一个滑动窗口的最大值，则将其从队列中删除
            if let Some(n) = deque.front() {
                if *n == nums[i-1] {
                    deque.pop_front();
                }
            }

            Self::push_element(nums[j], &mut deque);
            result.push(*deque.front().unwrap());
            i += 1;
            j += 1;
        }

        result
    }

    // 在队列添加一个元素，保证队列非严格单调减
    fn push_element(num: i32, deque: &mut VecDeque<i32>) {
        while match deque.back() {
            Some(n) if *n < num => true,
            _ => false
        } {
            deque.pop_back();
        }
        deque.push_back(num);
    }
}

fn main() {
    let nums = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;
    println!("{:?}", Solution::max_sliding_window(nums, k));
}
