struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
    depth: usize,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {stack: vec![], min_stack: vec![], depth: 0}
    }
    
    fn push(&mut self, x: i32) {
        if x <= Self::min(&self) || self.min_stack.is_empty() {
            self.min_stack.push(x);
        }
        self.stack.push(x);
        self.depth += 1;
    }
    
    fn pop(&mut self) {
        self.depth -= 1;
        let val = self.stack.pop().unwrap();
        if val == Self::min(&self) {
            self.min_stack.pop();
        }
    }
    
    fn top(&self) -> i32 { 
        if self.stack.is_empty() {-1}
        else {self.stack[self.depth-1]}   
    }
    
    fn min(&self) -> i32 {
        if self.min_stack.is_empty() {-1}
        else {self.min_stack[self.min_stack.len()-1]}
    }
}


fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    println!("{}", min_stack.min());
    min_stack.pop();
    println!("{}", min_stack.top());
    println!("{}", min_stack.min());
}