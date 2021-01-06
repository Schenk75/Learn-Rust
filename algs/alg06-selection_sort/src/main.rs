/// 选择排序
/// 时间复杂度:O(n2)，原地排序算法, 不稳定排序算法

fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return nums; }

    for i in 0..nums.len() {
        let mut min_idx = i;
        for j in i+1..nums.len() {
            if nums[j] < nums[min_idx] {
                min_idx = j;
            }
        }

        let tmp = nums[i];
        nums[i] = nums[min_idx];
        nums[min_idx] = tmp;
    }

    nums
}

fn main() {
    let nums = vec![4, 44, 5, 6, 1, 2, 3];
    println!("{:?}", selection_sort(nums));
}
