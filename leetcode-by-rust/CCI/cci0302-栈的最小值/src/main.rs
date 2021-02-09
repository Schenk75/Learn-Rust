struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>
}


impl MinStack {
    fn new() -> Self {
        MinStack {stack: vec![], min_stack: vec![]}
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);

        while let Some(val) = self.min_stack.last() {
            if *val < x {
                return;
            } else {
                break;
            }
        }
        // 包括min_stack中没有元素的情况
        self.min_stack.push(x);
    }
    
    fn pop(&mut self) {
        let tmp = self.stack.pop();
        if tmp.as_ref() == self.min_stack.last() {
            self.min_stack.pop();
        }
    }
    
    fn top(&self) -> i32 {
        if let Some(res) = self.stack.last() {
            *res
        } else {
            -1
        }
    }
    
    fn get_min(&self) -> i32 {
        if let Some(res) = self.min_stack.last() {
            *res
        } else {
            -1
        }
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    let ret_1 = obj.top();
    println!("{}", ret_1);
    let ret_2 = obj.get_min();
    println!("{}", ret_2);
    obj.pop();
    let ret_3 = obj.top();
    let ret_4 = obj.get_min();
    println!("{}", ret_3);
    println!("{}", ret_4);
}
