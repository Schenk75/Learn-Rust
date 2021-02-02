struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        // println!("len:{}", len);
        if len < 2 {return 0;}

        let mut nums = nums;
        let mut result = 0;
        Self::merge_sort(&mut nums, 0, len-1, &mut result);
        result
    }

    // 返回逆序数对数
    fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize, result: &mut i32) {
        if left >= right {return;}
        let mid = left + (right - left) / 2;
        // println!("mid:{}", mid);

        Self::merge_sort(nums, left, mid, result);         // 左半部分排序
        Self::merge_sort(nums, mid+1, right, result);      // 右半部分排序

        Self::merge(nums, left, mid, right, result);
    }

    // 左右两半数组都是有序的情况进行归并排序
    fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize, result: &mut i32) {
        let tmp = nums.clone();
        let (mut i, mut j, mut k) = (left, mid+1, left);
        
        while i <= mid && j <= right {
            if tmp[i] <= tmp[j] {
                nums[k] = tmp[i];
                i += 1;
            } else {
                nums[k] = tmp[j];
                j += 1;
                *result += (mid - i + 1) as i32;
            }
            k += 1;
        }
        while i <= mid {
            nums[k] = tmp[i];
            i += 1;
            k += 1;
        }
        while j <= right {
            nums[k] = tmp[j];
            j += 1;
            k += 1;
        }
        // println!("{:?}", nums);
    }
}

fn main() {
    let nums = vec![7,5,6,4];
    println!("{}", Solution::reverse_pairs(nums));
}
