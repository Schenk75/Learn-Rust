struct SortedStack {
    stack1: Vec<i32>,   // 大到小
    stack2: Vec<i32>,   // 小到大
}

impl SortedStack {

    fn new() -> Self {
        SortedStack {stack1: vec![], stack2: vec![]}
    }
    
    fn push(&mut self, val: i32) {
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }
        while !self.stack1.is_empty() && self.stack1.last().unwrap() < &val {
            self.stack2.push(self.stack1.pop().unwrap());
        }
        self.stack1.push(val);
    }
    
    fn pop(&mut self) {
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }
        if !self.stack1.is_empty() {
            self.stack1.pop();
        } else {
            return;
        }
    }
    
    fn peek(&mut self) -> i32 {
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }
        if !self.stack1.is_empty() {
            *self.stack1.last().unwrap()
        } else {
            -1
        }
    }
    
    fn is_empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

fn main() {
    let mut obj = SortedStack::new();
    obj.push(1);
    obj.push(2);
    obj.push(0);
    let ret_1 = obj.peek();
    obj.pop();
    let ret_2 = obj.peek();
    println!("{} {}", ret_1, ret_2);
}
