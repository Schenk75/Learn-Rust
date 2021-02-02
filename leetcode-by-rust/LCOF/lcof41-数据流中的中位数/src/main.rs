//// 插入时排序：时间复杂度为 O(N) ，其中包括： 查找元素插入位置 O(logN) （二分查找【我没有二分】）、向数组某位置插入元素 O(N)
// struct MedianFinder {
//     len: usize,
//     sorted_list: Vec<i32>,
// }

// impl MedianFinder {
//     /** initialize your data structure here. */
//     fn new() -> Self {
//         MedianFinder {len: 0, sorted_list: vec![]}
//     }
    
//     fn add_num(&mut self, num: i32) {
//         if self.sorted_list.is_empty() {
//             self.sorted_list.push(num);
//         } else {
//             for i in 0..self.len {
//                 if num < self.sorted_list[i] {
//                     self.sorted_list.insert(i, num);
//                     break;
//                 }
                
//                 if i == self.len - 1 {
//                     self.sorted_list.push(num);
//                 }
//             }

//         }
//         self.len += 1;
//     }
    
//     fn find_median(&self) -> f64 {
//         println!("{:?}", self.sorted_list);
//         // println!("{}", self.len);
//         if self.len % 2 == 1 {self.sorted_list[(self.len-1)/2] as f64}
//         else {((self.sorted_list[(self.len-1)/2] as f64) + (self.sorted_list[(self.len-1)/2 + 1] as f64)) / 2.0}
//     }
// }

use std::collections::BinaryHeap;

// 维护一个最大堆和一个最小堆
struct MedianFinder {
    heap_max: BinaryHeap<i32>,        // 存放小的那一半数
    heap_min: BinaryHeap<i32>,      // 存放大的那一半数
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {heap_max: BinaryHeap::new(), heap_min: BinaryHeap::new()}
    }
    
    fn add_num(&mut self, num: i32) {
        if self.heap_max.len() == self.heap_min.len() {
            self.heap_max.push(num);
            self.heap_min.push(-self.heap_max.pop().unwrap());
        } else {
            self.heap_min.push(-num);
            self.heap_max.push(-self.heap_min.pop().unwrap());
        }
        println!("{:?}, {:?}", self.heap_min, self.heap_max);
    }
    
    fn find_median(&self) -> f64 {
        if self.heap_max.len() == self.heap_min.len() {
            ((*self.heap_max.peek().unwrap() as f64) - (*self.heap_min.peek().unwrap() as f64)) / 2.0
        } else {
            -*self.heap_min.peek().unwrap() as f64
        }
    }
}


fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    let ret_1 = obj.find_median();
    obj.add_num(3);
    let ret_2 = obj.find_median();
    println!("{}, {}", ret_1, ret_2);
}
