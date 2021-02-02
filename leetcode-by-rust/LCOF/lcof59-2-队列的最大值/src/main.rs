use std::collections::VecDeque;

struct MaxQueue {
    queue: VecDeque<i32>,
    max_queue: VecDeque<i32>,      // 非严格单调减队列
}

impl MaxQueue {
    fn new() -> Self {
        MaxQueue {queue: VecDeque::new(), max_queue: VecDeque::new()}
    }
    
    fn max_value(&self) -> i32 {
        match self.max_queue.front() {
            Some(val) => *val,
            None => -1
        }
    }
    
    fn push_back(&mut self, value: i32) {
        self.queue.push_back(value);

        while match self.max_queue.back() {
            Some(val) if *val < value => true,
            _ => false
        } {
            self.max_queue.pop_back();
        }
        self.max_queue.push_back(value);
    }
    
    fn pop_front(&mut self) -> i32 {
        let result;
        match self.queue.pop_front() {
            Some(val) => result = val,
            None => return -1
        }

        if let Some(val) = self.max_queue.front() {
            if *val == result {
                self.max_queue.pop_front();
            }
        }

        result
    }
}


fn main() {
    let mut obj = MaxQueue::new();
    obj.push_back(1);
    obj.push_back(2);
    let ret1 = obj.max_value();
    let pop = obj.pop_front();
    let ret2 = obj.max_value();
    println!("{} {} {}", ret1, pop, ret2);
}
