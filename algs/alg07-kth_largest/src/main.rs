/// 找到第k大元素

// 假设有N个数存储在数组a中。我们从a中随机选择最后一个元素作为枢纽元，把数组分为两部分。其中左边元素都不比枢纽元大，右边元素都不比枢纽元小。此时枢纽元所在的位置记为mid。
// 如果右半边（包括a[mid]）的长度恰好为k，说明a[mid]就是需要的第k大元素，直接返回a[mid]。
// 如果右半边（包括a[mid]）的长度大于k，说明要寻找的第k大元素就在右半边，往右半边寻找。
// 如果右半边（包括a[mid]）的长度小于k，说明要寻找的第k大元素就在左半边，往左半边寻找。

fn kth_largest(mut nums: Vec<i32>, k: usize) -> Option<i32> {
    if nums.is_empty() || k >= nums.len() { return None; }

    let end = nums.len() - 1;
    
    // 分区点
    let mut pivot = partition(&mut nums, 0, end);
    while pivot + 1 != k {
        if k > pivot + 1 {
            pivot = partition(&mut nums, pivot + 1, end);
        } else {
            pivot = partition(&mut nums, 0, pivot - 1);
        }
    }
    Some(nums[pivot])
}

fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
    let pivot = nums[end];
    let mut mid = start;
    for j in start..end {
        if nums[j] >= pivot {
            swap(nums, mid, j);
            mid += 1;
        }
    }

    swap(nums, mid, end);
    mid
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    if i == j { return; }

    let tmp = nums[i];
    nums[i] = nums[j];
    nums[j] = tmp;
}

fn main() {
    let nums = vec![8, 10, 2, 3, 6,1, 5];
    println!("{:?}", kth_largest(nums, 3));
}
