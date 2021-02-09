struct MyQueue {
    p: Vec<i32>,
    q: Vec<i32>
}


impl MyQueue {
    fn new() -> Self {
        MyQueue {p: vec![], q: vec![]}
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.p.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.q.is_empty() {
            while !self.p.is_empty() {
                self.q.push(self.p.pop().unwrap());
            }
        }

        self.q.pop().unwrap()
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.q.is_empty() {
            while !self.p.is_empty() {
                self.q.push(self.p.pop().unwrap());
            }
        }

        *self.q.last().unwrap()
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        if self.p.is_empty() && self.q.is_empty() {true}
        else {false}
    }
}


fn main() {
    let mut queue = MyQueue::new();

    queue.push(1);
    queue.push(2);
    let ret_1 = queue.peek();  // 返回 1
    let ret_2 = queue.pop();   // 返回 1
    let ret_3 = queue.empty(); // 返回 false

    println!("{}, {}, {}", ret_1, ret_2, ret_3);
}
