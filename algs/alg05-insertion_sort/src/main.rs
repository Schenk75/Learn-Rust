/// 插入排序
/// 时间复杂度:O(n2)，原地排序算法, 稳定排序算法

fn insert_on_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return nums; }

    for i in 1..nums.len() {
        let val = nums[i];
        let mut j = (i - 1) as i32;
        while j >= 0 {
            if val < nums[j as usize] {
                nums[(j+1) as usize] = nums[j as usize];
            } else {
                break;
            }
            
            j -= 1;
        }
        nums[(j+1) as usize] = val;
    }

    nums
}

fn main() {
    let nums = vec![4, 5, 6,33,1, 2, 3];
    println!("{:?}", insert_on_sort(nums));
}
