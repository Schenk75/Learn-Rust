struct Solution;

impl Solution {
    // 二分查找找到target后再线性搜索数量
    pub fn search_(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {return 0;}
        let mut count = 0;
        let (mut left, mut right) = (0, (nums.len()-1) as i32);
        let mut mid = 0;
        let mut flag = false;
        
        // 二分查找
        while left <= right {
            mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                flag = true;
                break;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        // println!("{} {}", mid, flag);

        if flag {
            let mid = mid as usize;
            count += 1;
            let mut i = 1;
            while mid >= i {
                if nums[mid-i] == target {
                    count += 1;
                    i += 1;
                }
                else {break;}
            }
            i = 1;
            while mid + i < nums.len() {
                if nums[mid+i] == target {
                    count += 1;
                    i += 1;
                } else {
                    break;
                }
            }
            count
        } else {
            0
        }
    }

    // 二分查找查找边界
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {return 0;}
        let (mut left, mut right) = (0, (nums.len()-1) as i32);
        let (r_board, l_board);
        // 搜索右边界
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] <= target {left = mid + 1;}
            else {right = mid - 1;}
        }
        r_board = left;
        // println!("{}", right);
        // 如果查找不到，提前退出
        if right < 0 || nums[right as usize] != target {return 0;}
        
        // 搜索左边界
        left = 0;
        right = (nums.len()-1) as i32;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {right = mid - 1;}
            else {left = mid + 1;}
        }
        l_board = right;

        r_board - l_board - 1
    }
}

fn main() {
    let nums = vec![];
    let target = 9;
    println!("{}", Solution::search_(nums.clone(), target));
    println!("{}", Solution::search(nums.clone(), target));
}
