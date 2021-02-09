struct StackOfPlates {
    stack: Vec<Vec<i32>>,
    capacity: usize
}

impl StackOfPlates {
    fn new(cap: i32) -> Self {
        StackOfPlates {stack: vec![vec![]], capacity: cap as usize}
    }
    
    fn push(&mut self, val: i32) {
        if self.capacity == 0 {return;}
        let len = self.stack.len();
        if len != 0 && self.stack[len-1].len() < self.capacity {
            self.stack[len-1].push(val);
        } else {
            self.stack.push(vec![val]);
        }
    }
    
    fn pop(&mut self) -> i32 {
        let len = self.stack.len();
        if self.stack.is_empty() || self.stack[0].is_empty() {return -1;}
        let res = self.stack[len-1].pop().unwrap();
        if self.stack[len-1].is_empty() {
            self.stack.pop();
        }

        res
    }
    
    fn pop_at(&mut self, index: i32) -> i32 {
        if self.stack.len() <= index as usize || self.stack[0].is_empty() {
            return -1;
        }

        let res = self.stack[index as usize].pop().unwrap();
        if self.stack[index as usize].is_empty() {
            self.stack.remove(index as usize);
        }
        res
    }
}


fn main() {
    let mut obj = StackOfPlates::new(1);
    obj.push(1);
    obj.push(2);
    let ret_1 = obj.pop_at(1);
    let ret_2 = obj.pop();
    let ret_3 = obj.pop();

    println!("{} {} {}", ret_1, ret_2, ret_3);
}
