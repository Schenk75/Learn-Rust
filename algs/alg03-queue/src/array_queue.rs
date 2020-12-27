use std::fmt;

#[derive(Debug)]
pub struct ArrayQueue {
    queue: Vec<i32>,
    head: usize, 
    tail: usize,
}

impl ArrayQueue {
    pub fn new(n: usize) -> Self {
        ArrayQueue {
            queue: Vec::with_capacity(n),
            head: 0,
            tail: 0,
        }
    }

    // 添加元素从队列尾加
    pub fn enqueue(&mut self, num: i32) -> bool {
        let c = self.queue.capacity();
        // 队列满
        if self.head == 0 && self.tail == c {return false;}
        // 队列尾达到最大值，队列头不在0
        if self.tail == c {
            for i in 0..(self.tail-self.head) {
                self.queue[i] = self.queue[self.head+i];
            }
            self.tail -= self.head;
            self.head = 0;
            self.queue[self.tail] = num;
        } 
        // 队列尾不在最大值，队列头未知
        else {
            self.queue.push(num);
        }
        self.tail += 1;
        true
    }

    // 去掉元素从队列头删
    pub fn dequeue(&mut self) -> Option<i32> {
        if self.head == self.tail {return None;}

        let shift = self.queue[self.head];
        self.head += 1;
        Some(shift)
    }

}

impl fmt::Display for ArrayQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut result = String::new();
        for i in self.head..self.tail {
            if result.is_empty() {
                result = format!("{}", self.queue[i]); 
            } else {
                result = format!("{}->{}", result, self.queue[i]);
            }
        }
        
        write!(f, "{} head:{} tail:{}", result, self.head, self.tail)
    }
}