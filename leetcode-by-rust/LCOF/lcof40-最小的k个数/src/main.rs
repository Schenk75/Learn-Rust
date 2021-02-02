use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn get_least_numbers_sort(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();
        let mut result = vec![];
        result.append(&mut arr[0..(k as usize)].to_vec());
        result
    }

    // 使用最大堆实现
    pub fn get_least_numbers_heap(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(k as usize);

        for num in arr {
            if heap.len() < k as usize {
                heap.push(num);
            }
            else {
                if !heap.is_empty() && heap.peek().unwrap() > &num {
                    heap.pop();
                    heap.push(num);
                }
            }
        }

        let mut result = vec![];
        for item in heap {
            result.push(item);
        }
        result
    }

    // 快排变形
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        let k = k - 1;
        let mut start: i32 = 0;
        let mut end: i32 = (arr.len() - 1) as i32;
        let mut partition_index = -1;

        while partition_index != k {
            partition_index = Self::partition(&mut arr, start as usize, end as usize);
            if partition_index < k {
                start = partition_index + 1;
            } else {
                end = partition_index - 1;
            }
        }
        arr[..((k + 1) as usize)].iter().cloned().collect()
      }
    
    fn partition(arr: &mut Vec<i32>, start: usize, end: usize) -> i32 {
        let pivot = arr[end];
        let mut partition_index = start;

        for i in start..end {
            if arr[i] < pivot {
                arr.swap(i, partition_index);
                partition_index += 1;
            }
        }
        arr.swap(partition_index, end);
        partition_index as i32
    }
}

fn main() {
    let arr = vec![3,2,1];
    let k = 2;
    println!("{:?}", Solution::get_least_numbers_sort(arr.clone(), k));
    println!("{:?}", Solution::get_least_numbers_heap(arr.clone(), k));
    println!("{:?}", Solution::get_least_numbers(arr.clone(), k));
}
