// 冒泡排序
// 时间复杂度:O(n2)，原地排序算法, 稳定排序算法

fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {return Vec::new();}

    let length = nums.len();
    // 每次循环把最大值放到最后
    for i in 0..length {
        // 提前退出标志
        let mut swap = false;
        for j in 0..length-i-1 {
            if nums[j] > nums[j+1] {
                // 本次排序有数据交换
                swap = true;
                let tmp = nums[j];
                nums[j] = nums[j+1];
                nums[j+1] = tmp;
            }
        }
        // 若一轮排序没有数字交换，则提前退出
        if !swap {break;}
    }

    nums
}

fn main() {
    let nums = vec![4, 5, 6, 1, 2, 3];
    println!("{:?}", bubble_sort(nums));
}
