use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let len = nums.len() / 2;
        // println!("len:{}", len);

        for num in nums.iter() {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
            // println!("{}:{}", num, count);
            if *count > len {return *num;}
        }

        -1
    }

    pub fn majority_element_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        // 中心位置的数一定是众数
        nums[nums.len() / 2]
    }

    // 摩尔投票法
    pub fn majority_element_vote(nums: Vec<i32>) -> i32 {
        let mut vote = 0;
        let mut major = 0;

        for num in nums {
            if vote == 0 {major = num;}

            if num == major {
                vote += 1;
            } else {
                vote -= 1;
            }
        }

        major
    }
}

fn main() {
    let nums = vec![2,2,1,1,1,2,2];
    println!("{}", Solution::majority_element(nums.clone()));
    println!("{}", Solution::majority_element_sort(nums.clone()));
    println!("{}", Solution::majority_element_vote(nums.clone()));
}
